use quote::quote;
use syn::Ident;

pub(crate) fn generate_registry_entry(is_async: bool, crate_path: &proc_macro2::TokenStream, fn_name: &Ident, inner_fn_name: &Ident) -> proc_macro2::TokenStream {
  if is_async {
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
  }
}
