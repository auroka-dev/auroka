use super::super::auroka_test_runner::auroka_test_runner_env_set;
use super::super::TestMode;
use std::process::Command;

pub fn cargo_nextest_command(mode: TestMode) -> Command {
    let mut command = Command::new("cargo");

    command.args(&["nextest", "run"]);

    return auroka_test_runner_env_set(mode, command);
}
