use syn::{Expr, ExprCall, Result, parse::ParseStream};

pub(crate) fn parse_setup_step(input: ParseStream) -> Result<ExprCall> {
  let expr: Expr = input.parse()?;
  if let Expr::Call(call) = expr { Ok(call) } else { Err(syn::Error::new_spanned(expr, "Expected a function call for setup step")) }
}
