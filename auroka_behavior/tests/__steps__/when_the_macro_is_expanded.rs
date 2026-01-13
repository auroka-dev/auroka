use super::Context;
use crate::__helpers__::create_test_package;
use auroka_utils::Host;
use auroka_utils_packages::{Environment, PackageBuilder};

pub fn when_the_macro_is_expanded(context: &mut Context) {
  let (folder, package) = create_test_package(
    context.invocation().as_ref().unwrap(),
    &context
      .data()
      .iter()
      .map(|(k, v)| (k.clone(), v.clone()))
      .collect::<Vec<_>>(),
  );

  let mut package_builder = PackageBuilder::try_new(&folder, package).unwrap();

  package_builder
    .expand_test_target(Environment::Default, &Host::host())
    .unwrap();

  context.expansion_set(package_builder.output().clone());
  context.error_set(package_builder.error().clone());
}
