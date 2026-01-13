use syn::{Expr, Stmt};

pub(crate) fn process_statement(is_async: bool, stmt: &Stmt) -> Stmt {
  if let Stmt::Expr(expr, _semi) = stmt {
    if let Expr::Call(call) = expr {
      let mut call = call.clone();
      call.args.insert(0, syn::parse_quote!(&context));
      let expr = if is_async { syn::parse_quote!(#call.await?) } else { syn::parse_quote!(#call) };
      return Stmt::Expr(expr, Some(syn::token::Semi::default()));
    }
  }
  stmt.clone()
}
