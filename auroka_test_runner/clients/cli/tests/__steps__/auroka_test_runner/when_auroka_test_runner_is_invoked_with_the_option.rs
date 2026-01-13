use crate::__steps__::{Context, TestMode, auroka_test_runner::auroka_test_runner_command};

pub fn when_auroka_test_runner_is_invoked_with_the_option(context: &mut Context, argument: &str) -> anyhow::Result<()> {
  let mut command = auroka_test_runner_command(TestMode::Default);

  command.arg(argument);

  context.output_set(command.output());

  Ok(())
}
