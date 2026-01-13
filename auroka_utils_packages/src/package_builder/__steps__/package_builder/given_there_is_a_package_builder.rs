use super::super::Context;
use crate::{Package, PackageBuilder};
use auroka_utils::FileBuffer;
use rand::Rng;
use rand::distr::Alphanumeric;

pub fn given_there_is_a_package_builder(context: &mut Context) -> anyhow::Result<()> {
  let rng = rand::rng();
  let suffix: String = rng
    .sample_iter(&Alphanumeric)
    .take(5)
    .map(char::from)
    .collect();

  let mut package = Package::new("test");
  package.add_file(FileBuffer::new("src/lib.rs", "pub fn main() {}"));

  let package_builder = PackageBuilder::try_new(&format!("package_builder_tests_{}", suffix), package)?;

  context.package_builder_set(package_builder);
  Ok(())
}
