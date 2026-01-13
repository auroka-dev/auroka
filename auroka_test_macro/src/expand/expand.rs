use super::{generate_inner_function, generate_registry_entry, generate_test_wrapper, resolve_crate_path};
use quote::quote;
use syn::ItemFn;

pub(crate) fn expand(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
  let input_fn = match syn::parse2::<ItemFn>(item) {
    Ok(f) => f,
    Err(e) => return e.to_compile_error(),
  };

  let fn_name = &input_fn.sig.ident;
  let fn_body = &input_fn.block;
  let fn_vis = &input_fn.vis;
  let fn_ret = &input_fn.sig.output;
  let is_async = input_fn.sig.asyncness.is_some();

  let crate_path = resolve_crate_path();

  let inner_fn_name = syn::Ident::new(&format!("{}_inner", fn_name), fn_name.span());

  let inner_fn = generate_inner_function(is_async, fn_vis, &inner_fn_name, fn_ret, fn_body);

  let test_wrapper = generate_test_wrapper(is_async, fn_vis, fn_name, fn_ret, &inner_fn_name);

  let registry_entry = generate_registry_entry(is_async, &crate_path, fn_name, &inner_fn_name);

  quote! {
    #inner_fn
    #test_wrapper
    #registry_entry
  }
}
