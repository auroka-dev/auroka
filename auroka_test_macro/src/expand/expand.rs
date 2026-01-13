use super::{generate_inner_function, generate_registry_entry, generate_test_wrapper, resolve_crate_path};
use quote::quote;
use syn::ItemFn;

pub(crate) fn expand(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
  let input_fn = match syn::parse2::<ItemFn>(item) {
    Ok(f) => f,
    Err(e) => return e.to_compile_error(),
  };

  let fn_name = &input_fn.sig.ident;
  let is_async = input_fn.sig.asyncness.is_some();
  let inner_fn_name = syn::Ident::new(&format!("{}_inner", fn_name), fn_name.span());
  let crate_path = resolve_crate_path();

  let inner_fn = generate_inner_function(is_async, &input_fn.vis, &inner_fn_name, &input_fn.sig.output, &input_fn.block);
  let test_wrapper = generate_test_wrapper(is_async, &input_fn.vis, fn_name, &input_fn.sig.output, &inner_fn_name);
  let registry_entry = generate_registry_entry(is_async, &crate_path, fn_name, &inner_fn_name);

  quote! {
    #inner_fn
    #test_wrapper
    #registry_entry
  }
}
