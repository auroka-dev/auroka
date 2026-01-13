use auroka_utils::{FileBuffer, Host};
use auroka_utils_packages::{Dependency, Environment, Package, PackageBuilder};
use rand::{Rng, distr::Alphanumeric};
use std::collections::HashMap;

pub fn expand_in_temporary_package(invocation: &str, data: &HashMap<String, String>) -> PackageBuilder {
  let suffix: String = rand::rng()
    .sample_iter(&Alphanumeric)
    .take(5)
    .map(char::from)
    .collect();

  let mut package = Package::new("test");

  let mut auroka = Dependency::from_member("auroka", "auroka");
  auroka.set_default_features(false);
  package.add_dependency(auroka);

  package.add_file(FileBuffer::new("src/lib.rs", invocation));

  data.iter().for_each(|(file_name, content)| {
    package.add_file(FileBuffer::new(format!("src/__data__/{}", file_name), content.clone()));
  });

  let mut builder = PackageBuilder::try_new(&format!("macro_test_{}", suffix), package).unwrap();

  builder
    .expand_test_target(Environment::Default, &Host::host())
    .unwrap();

  builder
}
