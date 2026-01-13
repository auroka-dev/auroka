use super::super::Context;
use std::assert_matches::assert_matches;

pub fn then_the_standard_output_should_be_empty(context: &Context) -> anyhow::Result<()> {
  assert_matches!(context.output(), Some(_), "Expected Standard Output to be Some(_) but got {:?}", context.output());

  let obtained = context.output().clone().unwrap();

  assert!(obtained.is_empty(), "Expected Standard Output to be empty but got '{}'", obtained);
  Ok(())
}
