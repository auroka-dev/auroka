use crate::Outcome;
use quote::quote;
use syn::{Ident, Stmt};

pub(crate) fn generate_single_test(is_async: bool, fn_name: &Ident, setup_stmts: &[Stmt], outcomes: &[Outcome]) -> proc_macro2::TokenStream {
  let async_token = if is_async { quote!(async) } else { quote!() };
  let return_type = quote!(-> anyhow::Result<()>);

  let block = &outcomes[0].block();
  let stmts = &block.stmts;

  quote! {
      #[auroka::test]
      #async_token fn #fn_name() #return_type {
          let mut context = Context::new();
          #( #setup_stmts )*
          #( #stmts )*
          Ok(())
      }
  }
}
