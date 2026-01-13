use crate::__steps__::Context;
use crate::__steps__::given_there_is_a_macro_invocation;
use crate::__steps__::then_the_macro_expansion_should_have;
use crate::__steps__::then_the_standard_error_should_not_have;
use crate::__steps__::when_the_macro_is_expanded;

#[test]
pub fn expands_as_one_test_behavior() {
  let mut context = Context::new();

  given_there_is_a_macro_invocation(
    &mut context,
    r#"
behavior! {
  given_there_is_something()
  when_something_happens()

  "Something is true" {
    then_something_should_be_true()
  }

  "Something else is true" {
    then_something_else_should_be_true()
  }
}"#,
  );

  when_the_macro_is_expanded(&mut context);

  then_the_standard_error_should_not_have(&context, "error:");

  then_the_macro_expansion_should_have(
    &context,
    r#"
fn compact_inner() -> anyhow::Result<()> {
    let mut context = Context::new();
    given_there_is_something(&mut context);
    when_something_happens(&mut context);
    let mut _errors_ = Vec::new();
    if let Err(err) = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            then_something_should_be_true(&context);
        }),
    ) {
        _errors_.push(err);
    }
    if let Err(err) = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            then_something_else_should_be_true(&context);
        }),
    ) {
        _errors_.push(err);
    }
    if !_errors_.is_empty() {
        std::panic::resume_unwind(_errors_.remove(0));
    }
    Ok(())
}
"#,
  );
}
