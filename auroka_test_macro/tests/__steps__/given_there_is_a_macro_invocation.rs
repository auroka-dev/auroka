use super::Context;

pub fn given_there_is_a_macro_invocation(context: &mut Context, invocation: &str) -> anyhow::Result<()> {
  context.invocation_set(invocation);
  Ok(())
}
