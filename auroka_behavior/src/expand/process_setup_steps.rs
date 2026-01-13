pub(crate) fn process_setup_steps(behavior: &mut crate::Behavior) -> Vec<syn::Stmt> {
  let is_async = behavior.is_async();
  let mut setup_stmts = Vec::new();

  for step in behavior.setup_steps_mut() {
    step.args.insert(0, syn::parse_quote!(&mut context));
    let expr = if is_async { syn::parse_quote!(#step.await?) } else { syn::parse_quote!(#step?) };
    setup_stmts.push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
  }

  setup_stmts
}
