use super::process_statement;

pub(crate) fn process_outcomes(behavior: &mut crate::Behavior) {
  let is_async = behavior.is_async();

  for outcome in behavior.outcomes_mut() {
    let mut new_stmts = Vec::new();
    for stmt in &outcome.block().stmts {
      new_stmts.push(process_statement(is_async, stmt));
    }
    outcome.block_mut().stmts = new_stmts;
  }
}
