use std::assert_matches::assert_matches;

use super::super::Context;

pub fn then_the_result_should_be_ok(context: &Context) {
  let result = context.result();

  assert_matches!(
    result,
    Ok(_),
    "Expected build result to be Ok(_) but got {:?}",
    result
  );
}
