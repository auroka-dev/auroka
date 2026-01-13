use super::Context;
use auroka_assertions::assert_contains_diff;
use std::assert_matches::assert_matches;

pub fn then_the_macro_expansion_should_have(context: &Context, expansion: &str) {
  assert_matches!(context.expansion(), Some(_), "Expected macro expansion to be set");

  let obtained = context.expansion().as_ref().unwrap();

  assert_contains_diff!(obtained, expansion, "Expected macro expansion to contain '{}'", expansion);
}
