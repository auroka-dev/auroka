use crate::__steps__::Context;
use crate::__steps__::given_there_is_a_macro_invocation;
use crate::__steps__::then_the_macro_expansion_should_have;
use crate::__steps__::then_the_standard_error_should_not_have;
use crate::__steps__::when_the_macro_is_expanded;

#[test]
pub fn expands_worker_test_behavior() {
  let mut context = Context::new();

  given_there_is_a_macro_invocation(
    &mut context,
    r#"
behavior! {
  given_there_is_something("boo", "cool")
  when_something_happens("boo", "cool")

  "Something is true" {
    then_something_should_be_true("boo", "cool")
  }
}"#,
  );
  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_not_have(&context, "error:");

  then_the_macro_expansion_should_have(
    &context,
    r#"
fn something_is_true() {
    let mut context = Context::new();
    given_there_is_something(&mut context, "boo", "cool");
    when_something_happens(&mut context, "boo", "cool");
    then_something_should_be_true(&context, "boo", "cool");
}
"#,
  );
}
