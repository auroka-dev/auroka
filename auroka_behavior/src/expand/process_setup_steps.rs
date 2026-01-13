use syn::ExprCall;

pub(crate) fn process_setup_steps(is_async: bool, setup_steps: &mut [ExprCall]) -> Vec<syn::Stmt> {
  let mut setup_stmts = Vec::new();
  for step in setup_steps {
    step.args.insert(0, syn::parse_quote!(&mut context));
    let expr = if is_async { syn::parse_quote!(#step.await?) } else { syn::parse_quote!(#step) };
    setup_stmts.push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
  }
  setup_stmts
}
