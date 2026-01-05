use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn auroka_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input_fn = parse_macro_input!(item as ItemFn);
  let fn_name = &input_fn.sig.ident;
  let fn_body = &input_fn.block;
  let fn_vis = &input_fn.vis;
  let fn_ret = &input_fn.sig.output;
  let is_async = input_fn.sig.asyncness.is_some();

  // Create a unique name for the inner function containing the actual logic
  let inner_fn_name = syn::Ident::new(&format!("{}_inner", fn_name), fn_name.span());

  // 1. The Inner Function (The actual test logic)
  // We preserve the async-ness of the original function
  let inner_fn = if is_async {
    quote! {
      #fn_vis async fn #inner_fn_name() #fn_ret #fn_body
    }
  } else {
    quote! {
      #fn_vis fn #inner_fn_name() #fn_ret #fn_body
    }
  };

  // 2. The Standard Test Wrapper (For `cargo test`)
  // This allows the test to still run with standard `cargo test` on native
  let test_wrapper = if is_async {
    quote! {
      #[cfg(not(target_arch = "wasm32"))]
      #[test]
      #fn_vis fn #fn_name() #fn_ret {
          tokio::runtime::Builder::new_current_thread()
              .enable_all()
              .build()
              .unwrap()
              .block_on(#inner_fn_name())
      }
    }
  } else {
    quote! {
      #[cfg(not(target_arch = "wasm32"))]
      #[test]
      #fn_vis fn #fn_name() #fn_ret {
          #inner_fn_name()
      }
    }
  };

  // 3. The Registry Entry
  // We normalize everything to a Boxed Future so the runner can just `await` it.
  let registry_entry = if is_async {
    quote! {
      auroka_test::inventory::submit! {
        auroka_test::Test {
            name: stringify!(#fn_name),
            test_fn: || Box::pin(#inner_fn_name()),
        }
      }
    }
  } else {
    quote! {
      auroka_test::inventory::submit! {
        auroka_test::Test {
            name: stringify!(#fn_name),
            test_fn: || Box::pin(async { #inner_fn_name() }),
        }
      }
    }
  };

  let output = quote! {
    #inner_fn
    #test_wrapper
    #registry_entry
  };

  output.into()
}
