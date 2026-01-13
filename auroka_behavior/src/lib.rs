use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
  Block, Expr, ExprCall, LitStr, Result,
  parse::{Parse, ParseStream},
  parse_macro_input,
};

struct BehaviorInput {
  is_async: bool,
  setup_steps: Vec<ExprCall>,
  test_cases: Vec<TestCase>,
}

struct TestCase {
  name: LitStr,
  block: Block,
}

impl Parse for BehaviorInput {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut is_async = false;
    if input.peek(syn::Token![:]) {
      input.parse::<syn::Token![:]>()?;
      input.parse::<syn::Token![async]>()?;
      is_async = true;
    }

    let mut setup_steps = Vec::new();
    let mut test_cases = Vec::new();

    while !input.is_empty() {
      if input.peek(LitStr) {
        // Parse a test case: "Name" { ... }
        let name: LitStr = input.parse()?;
        let block: Block = input.parse()?;
        test_cases.push(TestCase { name, block });
      } else {
        // Parse a setup step: function_call()
        // We parse it as an expression first to handle the call syntax
        let expr: Expr = input.parse()?;
        if let Expr::Call(call) = expr {
          setup_steps.push(call);
        } else {
          return Err(syn::Error::new_spanned(expr, "Expected a function call for setup step"));
        }
      }
    }

    Ok(BehaviorInput { is_async, setup_steps, test_cases })
  }
}

#[proc_macro]
pub fn behavior(input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as BehaviorInput);
  let is_async = input.is_async;

  let mut setup_stmts = Vec::new();
  for mut step in input.setup_steps {
    step.args.insert(0, syn::parse_quote!(&mut context));
    let expr = if is_async { syn::parse_quote!(#step.await?) } else { syn::parse_quote!(#step) };
    setup_stmts.push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
  }

  for case in &mut input.test_cases {
    let mut new_stmts = Vec::new();
    for stmt in &case.block.stmts {
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
    case.block.stmts = new_stmts;
  }

  let test_cases = &input.test_cases;
  let fn_name_str = if test_cases.is_empty() {
    "no_outcome".to_string()
  } else if test_cases.len() == 1 {
    test_cases[0]
      .name
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
  if test_cases.len() == 1 {
    let block = &test_cases[0].block;
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
    return TokenStream::from(expanded);
  }

  // Case 2: Multiple test cases (expands_as_one_test_behavior.rs)
  if test_cases.len() > 1 {
    let mut checks = Vec::new();

    for case in test_cases {
      let block = &case.block;
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
    return TokenStream::from(expanded);
  }

  TokenStream::new()
}
