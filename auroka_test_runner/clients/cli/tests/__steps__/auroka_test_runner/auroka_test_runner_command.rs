use super::super::TestMode;
use super::auroka_test_runner_env_set;
use std::process::Command;

pub fn auroka_test_runner_command(mode: TestMode) -> Command {
  let exe = env!("CARGO_BIN_EXE_auroka_test_runner");
  let command = Command::new(exe);

  return auroka_test_runner_env_set(mode, command);
}
