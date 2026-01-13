use super::super::Context;
use std::assert_matches::assert_matches;

pub fn then_the_standard_output_should_have(context: &Context, content: &str) {
  assert_matches!(context.output(), Some(_), "Expected Standard Output to be Some(_) but got {:?}", context.output());

  let obtained = context.output().clone().unwrap();

  assert!(obtained.contains(content), "Expected Standard Output to contain '{}' but its {}", content, obtained);
}
