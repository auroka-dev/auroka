use quote::quote;
use syn::{Block, Ident, ReturnType, Visibility};

pub(crate) fn generate_inner_function(is_async: bool, fn_vis: &Visibility, inner_fn_name: &Ident, fn_ret: &ReturnType, fn_body: &Block) -> proc_macro2::TokenStream {
  if is_async {
    quote! {
      #fn_vis async fn #inner_fn_name() #fn_ret #fn_body
    }
  } else {
    quote! {
      #fn_vis fn #inner_fn_name() #fn_ret #fn_body
    }
  }
}
