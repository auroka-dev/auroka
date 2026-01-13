use quote::quote;
use syn::{Ident, ItemFn};

pub(crate) fn generate_registry_entry(fn_item: &ItemFn, crate_path: &proc_macro2::TokenStream, inner_fn_name: &Ident) -> proc_macro2::TokenStream {
  let fn_name = &fn_item.sig.ident;
  let is_async = fn_item.sig.asyncness.is_some();

  let call = if is_async {
    quote! { #inner_fn_name().await }
  } else {
    quote! { #inner_fn_name() }
  };

  quote! {
    const _: () = {
      #crate_path::inventory::submit! {
        #crate_path::Test {
            name: stringify!(#fn_name),
            test_fn: || {
                use #crate_path::TestReturn;
                Box::pin(async move {
                    #call.into_result()
                })
            },
        }
      }
    };
  }
}
