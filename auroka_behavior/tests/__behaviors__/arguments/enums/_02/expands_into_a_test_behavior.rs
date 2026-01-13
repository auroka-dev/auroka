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
  given_there_is_something(Ordering::Equal, Ordering::Equal)
  when_something_happens(Ordering::Equal, Ordering::Equal)

  "Something is true" {
    then_something_should_be_true(Ordering::Equal, Ordering::Equal)
  }
}"#,
  );

  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_not_have(&context, "error:");

  then_the_macro_expansion_should_have(
    &context,
    r#"
fn something_is_true_inner() -> anyhow::Result<()> {
    let mut context = Context::new();
    given_there_is_something(&mut context, Ordering::Equal, Ordering::Equal);
    when_something_happens(&mut context, Ordering::Equal, Ordering::Equal);
    then_something_should_be_true(&context, Ordering::Equal, Ordering::Equal);
    Ok(())
}
"#,
  );
}
