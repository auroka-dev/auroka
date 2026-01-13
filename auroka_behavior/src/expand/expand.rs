use crate::Behavior;
use quote::{format_ident, quote};
use syn::Expr;

#[doc(hidden)]
pub fn expand(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
  let mut input = syn::parse2::<Behavior>(input)?;
  let is_async = input.is_async();

  let mut setup_stmts = Vec::new();
  for step in input.setup_steps_mut() {
    step.args.insert(0, syn::parse_quote!(&mut context));
    let expr = if is_async { syn::parse_quote!(#step.await?) } else { syn::parse_quote!(#step) };
    setup_stmts.push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
  }

  for outcome in input.outcomes_mut() {
    let mut new_stmts = Vec::new();
    for stmt in &outcome.block().stmts {
      if let syn::Stmt::Expr(expr, _) = stmt {
        if let Expr::Call(call) = expr {
          let mut call = call.clone();
          call.args.insert(0, syn::parse_quote!(&context));
          let expr = if is_async { syn::parse_quote!(#call.await?) } else { syn::parse_quote!(#call) };
          new_stmts.push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
        } else {
          new_stmts.push(stmt.clone());
        }
      } else {
        new_stmts.push(stmt.clone());
      }
    }
    outcome.block_mut().stmts = new_stmts;
  }

  let outcomes = input.outcomes();
  let fn_name_str = if outcomes.is_empty() {
    "no_outcome".to_string()
  } else if outcomes.len() == 1 {
    outcomes[0]
      .name()
      .value()
      .to_lowercase()
      .replace(" ", "_")
      .replace("-", "_")
  } else {
    "compact".to_string()
  };
  let fn_name = format_ident!("{}", fn_name_str);

  let async_token = if is_async { quote!(async) } else { quote!() };
  let return_type = quote!(-> anyhow::Result<()>);

  // Case 1: Single test case (expands_test_behavior.rs)
  if outcomes.len() == 1 {
    let block = &outcomes[0].block();
    let stmts = &block.stmts;

    let expanded = quote! {
        #[auroka::test]
        #async_token fn #fn_name() #return_type {
            let mut context = Context::new();
            #( #setup_stmts )*
            #( #stmts )*
            Ok(())
        }
    };
    return Ok(expanded);
  }

  // Case 2: Multiple test cases (expands_as_one_test_behavior.rs)
  if outcomes.len() > 1 {
    let mut checks = Vec::new();

    for outcome in outcomes {
      let block = &outcome.block();
      let stmts = &block.stmts;
      // We wrap the block content in a closure for AssertUnwindSafe
      checks.push(quote! {
          if let Err(err) = std::panic::catch_unwind(
              std::panic::AssertUnwindSafe(|| {
                  #( #stmts )*
              }),
          ) {
              _errors_.push(err);
          }
      });
    }

    let expanded = quote! {
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
    };
    return Ok(expanded);
  }

  // Fallback for empty test cases or other scenarios
  Ok(proc_macro2::TokenStream::new())
}
