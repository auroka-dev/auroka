use crate::__steps__::Context;
use crate::__steps__::given_there_is_a_macro_invocation;
use crate::__steps__::then_the_standard_error_should_have;
use crate::__steps__::when_the_macro_is_expanded;

const INPUT: &str = r#"
behavior! {
  given_there_is_something()
  when_something_happens(boo)

  "Something is true" {
    then_something_should_be(cool)
  }
}
"#;

#[test]
pub fn returns_not_found_error_behavior() -> anyhow::Result<()> {
  let mut context = Context::new();
  given_there_is_a_macro_invocation(&mut context, INPUT)?;
  when_the_macro_is_expanded(&mut context)?;
  then_the_standard_error_should_have(&context, "error[E0425]: cannot find value `boo` in this scope")?;
  then_the_standard_error_should_have(&context, "error[E0425]: cannot find value `cool` in this scope")?;
  Ok(())
}
