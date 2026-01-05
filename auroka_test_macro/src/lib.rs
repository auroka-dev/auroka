use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
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

  // Resolve the correct path to auroka_test based on what the user imported
  let crate_path = match crate_name("auroka_test") {
    Ok(FoundCrate::Itself) => quote! { crate },
    Ok(FoundCrate::Name(name)) => {
      let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
      quote! { ::#ident }
    }
    Err(_) => {
      // If auroka_test is not found directly, try auroka
      match crate_name("auroka") {
        Ok(FoundCrate::Itself) => quote! { crate::auroka_test },
        Ok(FoundCrate::Name(name)) => {
          let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
          quote! { ::#ident::auroka_test }
        }
        Err(_) => {
          // Fallback to assuming auroka_test is in scope
          quote! { ::auroka_test }
        }
      }
    }
  };

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
  // Use the dynamically resolved crate path for proper hygiene
  let registry_entry = if is_async {
    quote! {
      const _: () = {
        #crate_path::inventory::submit! {
          #crate_path::Test {
              name: stringify!(#fn_name),
              test_fn: || {
                  use #crate_path::TestReturn;
                  Box::pin(async move {
                      #inner_fn_name().await.into_result()
                  })
              },
          }
        }
      };
    }
  } else {
    quote! {
      const _: () = {
        #crate_path::inventory::submit! {
          #crate_path::Test {
              name: stringify!(#fn_name),
              test_fn: || {
                  use #crate_path::TestReturn;
                  Box::pin(async move {
                      #inner_fn_name().into_result()
                  })
              },
          }
        }
      };
    }
  };

  let output = quote! {
    #inner_fn
    #test_wrapper
    #registry_entry
  };

  output.into()
}
