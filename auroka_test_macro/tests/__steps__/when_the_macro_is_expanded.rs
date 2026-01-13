use super::Context;
use auroka_utils::{FileBuffer, Host};
use auroka_utils_packages::{Dependency, Environment, Package, PackageBuilder};
use rand::{Rng, distr::Alphanumeric};

pub fn when_the_macro_is_expanded(context: &mut Context) {
  let rng = rand::rng();
  let suffix: String = rng
    .sample_iter(&Alphanumeric)
    .take(5)
    .map(char::from)
    .collect();

  let folder = format!("macro_auroka_test_tests_{}", suffix);

  let mut package = Package::new("test");

  let mut auroka = Dependency::from_member("auroka", "auroka");
  auroka.set_default_features(false);
  package.add_dependency(auroka);

  let invocation = context.invocation().as_ref().unwrap();

  package.add_file(FileBuffer::new("src/lib.rs", invocation));

  context.data().iter().for_each(|(file_name, content)| {
    package.add_file(FileBuffer::new(format!("src/__data__/{}", file_name), content.clone()));
  });

  let mut package_builder = PackageBuilder::try_new(&folder, package).unwrap();

  package_builder
    .expand_test_target(Environment::Default, &Host::host())
    .unwrap();

  context.expansion_set(package_builder.output().clone());
  context.error_set(package_builder.error().clone());
}
