use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
  Block, Expr, ExprCall, LitStr, Result,
  parse::{Parse, ParseStream},
  parse_macro_input,
};

struct BehaviorInput {
  setup_steps: Vec<ExprCall>,
  test_cases: Vec<TestCase>,
}

struct TestCase {
  name: LitStr,
  block: Block,
}

impl Parse for BehaviorInput {
  fn parse(input: ParseStream) -> Result<Self> {
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
          return Err(syn::Error::new_spanned(
            expr,
            "Expected a function call for setup step",
          ));
        }
      }
    }

    Ok(BehaviorInput {
      setup_steps,
      test_cases,
    })
  }
}

#[proc_macro]
pub fn behavior(input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as BehaviorInput);

  for step in &mut input.setup_steps {
    step.args.insert(0, syn::parse_quote!(&mut context));
  }

  for case in &mut input.test_cases {
    for stmt in &mut case.block.stmts {
      if let syn::Stmt::Expr(expr, semi) = stmt {
        if let Expr::Call(call) = expr {
          call.args.insert(0, syn::parse_quote!(&context));
        }
        if semi.is_none() {
          *semi = Some(syn::token::Semi::default());
        }
      }
    }
  }

  let setup_steps = &input.setup_steps;
  let test_cases = &input.test_cases;
  let test_case = &test_cases[0];
  let fn_name_str = test_case
    .name
    .value()
    .to_lowercase()
    .replace(" ", "_")
    .replace("-", "_");
  let fn_name = format_ident!("{}", fn_name_str);

  // Case 1: Single test case (expands_test_behavior.rs)
  if test_cases.len() == 1 {
    let block = &test_case.block;
    let stmts = &block.stmts;

    let expanded = quote! {
        #[test]
        fn #fn_name() {
            let mut context = Context::new();
            #( #setup_steps; )*
            #( #stmts )*
        }
    };
    return TokenStream::from(expanded);
  }

  // Case 2: Multiple test cases (expands_as_one_test_behavior.rs)
  // The test expects a single function named `compact_behavior` that runs all assertions
  // and aggregates errors using catch_unwind.
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
                #[test]
                fn #fn_name() {
                    let mut context = Context::new();
                    #( #setup_steps; )*
                    let mut _errors_ = Vec::new();

                    #( #checks )*

                    if !_errors_.is_empty() {
                      std::panic::resume_unwind(_errors_.remove(0));
                    }
                }
    };
    return TokenStream::from(expanded);
  }

  TokenStream::new()
}
