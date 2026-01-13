use crate::__steps__::OutputContext;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_output_should_have(context: &dyn OutputContext, content: &str) -> anyhow::Result<()> {
  let output = context.output().expect("No output was produced");

  println!("Asserting that standard output contains: {}", content);

  output.assert().stdout(str::contains(content));

  Ok(())
}
