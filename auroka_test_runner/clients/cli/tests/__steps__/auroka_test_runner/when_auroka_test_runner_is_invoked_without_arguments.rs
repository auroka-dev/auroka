use crate::__steps__::{Context, TestMode, auroka_test_runner::auroka_test_runner_command};

pub fn when_auroka_test_runner_is_invoked_without_arguments(context: &mut Context) {
  let mut command = auroka_test_runner_command(TestMode::Default);

  context.output_set(command.output());
}
