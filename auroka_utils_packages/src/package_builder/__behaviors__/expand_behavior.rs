use crate::package_builder::__steps__::{Context, given_there_is_a_package_builder, then_the_standard_output_should_have, when_expand_is_invoked};

#[test]
fn expand_behavior() -> anyhow::Result<()> {
  let mut context = Context::new();

  given_there_is_a_package_builder(&mut context)?;
  when_expand_is_invoked(&mut context)?;

  // "Outputs the expansion to the standard output"
  then_the_standard_output_should_have(
    &mut context,
    "#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
pub fn main() {}
",
  )?;
  Ok(())
}
