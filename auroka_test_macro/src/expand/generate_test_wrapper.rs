use quote::quote;
use syn::{Ident, ItemFn};

pub(crate) fn generate_test_wrapper(fn_item: &ItemFn, inner_fn_name: &Ident) -> proc_macro2::TokenStream {
  let is_async = fn_item.sig.asyncness.is_some();
  let fn_vis = &fn_item.vis;
  let fn_name = &fn_item.sig.ident;
  let fn_ret = &fn_item.sig.output;

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
