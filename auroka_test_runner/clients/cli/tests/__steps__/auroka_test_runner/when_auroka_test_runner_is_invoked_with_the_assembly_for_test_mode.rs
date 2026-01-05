use crate::__steps__::TestMode;
use crate::__steps__::{auroka_test_runner::auroka_test_runner_command, Context};

pub fn when_auroka_test_runner_is_invoked_with_the_assembly_for_test_mode(
    context: &mut Context,
    mode: TestMode,
) {
    let mut command = auroka_test_runner_command(mode);

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
