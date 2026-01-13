use super::generate_check::generate_check;
use crate::Outcome;
use quote::quote;
use syn::{Ident, Stmt};

pub(crate) fn generate_multi_test(is_async: bool, fn_name: &Ident, setup_stmts: &[Stmt], outcomes: &[Outcome]) -> proc_macro2::TokenStream {
  let async_token = if is_async { quote!(async) } else { quote!() };
  let return_type = quote!(-> anyhow::Result<()>);

  let checks: Vec<_> = outcomes.iter().map(generate_check).collect();

  quote! {
      #[auroka::test]
      #async_token fn #fn_name() #return_type {
          let mut context = Context::new();
          #( #setup_stmts )*
          let mut _errors_ = Vec::new();

          #( #checks )*

          if !_errors_.is_empty() {
            std::panic::resume_unwind(_errors_.remove(0));
          }

          Ok(())
      }
  }
}
