use quote::quote;
use syn::{Ident, ItemFn};

pub(crate) fn generate_inner_function(fn_item: &ItemFn, inner_fn_name: &Ident) -> proc_macro2::TokenStream {
  let is_async = fn_item.sig.asyncness.is_some();
  let fn_vis = &fn_item.vis;
  let fn_ret = &fn_item.sig.output;
  let fn_body = &fn_item.block;

  if is_async {
    return quote! {
      #fn_vis async fn #inner_fn_name() #fn_ret #fn_body
    };
  }

  quote! {
    #fn_vis fn #inner_fn_name() #fn_ret #fn_body
  }
}
