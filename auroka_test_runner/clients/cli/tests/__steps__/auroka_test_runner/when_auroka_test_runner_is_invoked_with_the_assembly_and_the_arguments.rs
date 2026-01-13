use crate::__steps__::{
    auroka_test_runner::auroka_test_runner_command, Context, TestMode,
};

pub fn when_auroka_test_runner_is_invoked_with_the_assembly_and_the_arguments(
    context: &mut Context,
    arguments: &str,
) -> anyhow::Result<()> {
    let mut command = auroka_test_runner_command(TestMode::Default);

    if arguments.starts_with("--list") && arguments.contains("--ignored") {
        command.arg(context.sandbox().original());
    } else {
        command.arg(context.sandbox_mut().assembly());
    }

    command.args(arguments.split_whitespace());

    context.output_set(command.output());
    Ok(())
}
