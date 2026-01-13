use super::process_statement;
use crate::Outcome;

pub(crate) fn process_outcomes(is_async: bool, outcomes: &mut [Outcome]) {
  for outcome in outcomes {
    let mut new_stmts = Vec::new();
    for stmt in &outcome.block().stmts {
      new_stmts.push(process_statement(is_async, stmt));
    }
    outcome.block_mut().stmts = new_stmts;
  }
}
