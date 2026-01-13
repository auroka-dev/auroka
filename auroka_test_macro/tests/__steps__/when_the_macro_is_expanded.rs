use crate::__helpers__::expand_in_temporary_package;
use crate::__steps__::Context;

pub fn when_the_macro_is_expanded(context: &mut Context) -> anyhow::Result<()> {
  let invocation = context
    .invocation()
    .as_ref()
    .expect("invocation must be set");

  let builder = expand_in_temporary_package(invocation, context.data());

  context.expansion_set(builder.output().clone());
  context.error_set(builder.error().clone());
  Ok(())
}
