use quote::quote;
use syn::{Ident, ReturnType, Visibility};

pub(crate) fn generate_test_wrapper(is_async: bool, fn_vis: &Visibility, fn_name: &Ident, fn_ret: &ReturnType, inner_fn_name: &Ident) -> proc_macro2::TokenStream {
  if is_async {
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
  }
}
