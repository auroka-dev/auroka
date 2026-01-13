use crate::package_builder::__steps__::{
  Context, given_there_is_a_package_builder, then_the_result_should_be_ok, then_the_standard_error_should_have, then_the_standard_error_should_not_have, then_the_standard_output_should_be_empty, when_build_is_invoked,
};

#[test]
pub fn builds_the_crate() -> anyhow::Result<()> {
  let mut context = Context::new();

  given_there_is_a_package_builder(&mut context)?;
  when_build_is_invoked(&mut context)?;

  // Outputs 'Finished' to the standard error"
  then_the_standard_error_should_have(&mut context, "    Finished ")?;

  // Outputs no errors to the standard error"
  then_the_standard_error_should_not_have(&mut context, "error:")?;

  // Outputs nothing to the standard output
  then_the_standard_output_should_be_empty(&mut context)?;

  // "Returns no error"
  then_the_result_should_be_ok(&mut context)?;
  Ok(())
}
