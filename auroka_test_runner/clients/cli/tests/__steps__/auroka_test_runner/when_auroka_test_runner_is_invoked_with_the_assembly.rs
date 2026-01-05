use crate::__steps__::{
    auroka_test_runner::auroka_test_runner_command, Context, TestMode,
};

pub fn when_auroka_test_runner_is_invoked_with_the_assembly(context: &mut Context) {
    let mut command = auroka_test_runner_command(TestMode::Default);

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
