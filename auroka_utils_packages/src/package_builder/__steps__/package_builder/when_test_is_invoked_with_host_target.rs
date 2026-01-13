use super::super::Context;
use auroka_utils::Host;

pub fn when_test_is_invoked_with_host_target(context: &mut Context) -> anyhow::Result<()> {
  let error;
  let output;
  let result;

  {
    let builder = context.package_builder_mut();
    result = builder.test_target(&Host::host());
    error = builder.error().clone();
    output = builder.output().clone();
  }

  context.result_set(result);

  context.error_set(error);
  context.output_set(output);
  Ok(())
}
