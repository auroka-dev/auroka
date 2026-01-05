use crate::__steps__::Context;
use crate::__steps__::given_there_is_a_macro_invocation;
use crate::__steps__::then_the_standard_error_should_have;
use crate::__steps__::when_the_macro_is_expanded;

#[test]
pub fn expands_worker_test_behavior() {
  let mut context = Context::new();

  given_there_is_a_macro_invocation(
    &mut context,
    r#"
behavior! {
  given_there_is_something()
  when_something_happens(boo)

  "Something is true" {
    then_something_should_be(cool)
  }
}
"#,
  );

  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_have(
    &context,
    "error[E0425]: cannot find value `boo` in this scope",
  );

  then_the_standard_error_should_have(
    &context,
    "error[E0425]: cannot find value `cool` in this scope",
  );
}
