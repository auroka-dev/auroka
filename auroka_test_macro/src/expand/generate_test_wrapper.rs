use quote::quote;
use syn::{Ident, ItemFn};

pub(crate) fn generate_test_wrapper(fn_item: &ItemFn, inner_fn_name: &Ident) -> proc_macro2::TokenStream {
  let (vis, name, ret) = (&fn_item.vis, &fn_item.sig.ident, &fn_item.sig.output);

  if fn_item.sig.asyncness.is_some() {
    quote! {
      #[cfg(not(target_arch = "wasm32"))]
      #[test]
      #vis fn #name() #ret {
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
      #vis fn #name() #ret {
          #inner_fn_name()
      }
    }
  }
}
