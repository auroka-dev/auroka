use crate::package_builder::__steps__::{Context, given_there_is_a_package_builder_with, then_the_standard_output_should_have, when_test_is_invoked_with_host_target};

#[test]
fn executes_failing_test_behavior() -> anyhow::Result<()> {
  let mut context = Context::new();

  given_there_is_a_package_builder_with(
    &mut context,
    r#"
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 2);
    }
}
"#,
  )?;

  when_test_is_invoked_with_host_target(&mut context)?;

  // "Outputs the test result to the standard output"
  then_the_standard_output_should_have(
    &mut context,
    r#"assertion `left == right` failed
  left: 1
 right: 2"#,
  )?;
  Ok(())
}
