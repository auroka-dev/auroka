use std::assert_matches::assert_matches;

use super::super::Context;

pub fn then_the_standard_error_should_have(context: &Context, content: &str) {
  assert_matches!(context.error(), Some(_), "Expected Standard Error to be Some(_) but got {:?}", context.error());

  let obtained = context.error().clone().unwrap();

  assert!(obtained.contains(content), "Expected Standard Error to contain '{}' but its {}", content, obtained);
}
