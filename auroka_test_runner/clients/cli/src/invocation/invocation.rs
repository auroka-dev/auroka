use super::styles;
use crate::OutputFormat;
use clap::Parser;

#[derive(Parser)]
#[command(name = "auroka_test_runner")]
#[command(about = "Execute all unit and integration tests and build examples of a local package")]
#[command(version)]
#[command(styles = styles())]
pub struct Invocation {
  /// The wasm file to test
  #[arg(value_name = "INPUT")]
  input: Option<String>,

  /// The filter string to match test names against
  #[arg(value_name = "FILTER")]
  filter: Option<String>,

  /// Include ignored tests in the test run
  #[arg(long)]
  include_ignored: bool,

  /// Run only ignored tests
  #[arg(long)]
  ignored: bool,

  /// Do not capture stdout/stderr
  #[arg(long)]
  nocapture: bool,

  /// List all tests that would be run
  #[arg(long)]
  list: bool,

  /// Exactly match filters rather than by substring
  #[arg(long)]
  exact: bool,

  /// Skip tests whose names match the given pattern
  #[arg(long, value_name = "PATTERN")]
  skip: Vec<String>,

  /// Configure formatting of output
  #[arg(long, value_name = "FORMAT")]
  format: Option<OutputFormat>,
}
