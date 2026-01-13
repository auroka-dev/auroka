use super::super::Context;
use std::assert_matches::assert_matches;

pub fn then_the_result_should_be_ok(context: &Context) -> anyhow::Result<()> {
  let result = context.result();

  assert_matches!(result, Ok(_), "Expected build result to be Ok(_) but got {:?}", result);
  Ok(())
}
