use super::Context;

pub fn given_the_assembly_path_is(context: &mut Context, assembly_path: &str) -> anyhow::Result<()> {
  context.assembly_path_set(assembly_path.to_string());
  Ok(())
}
