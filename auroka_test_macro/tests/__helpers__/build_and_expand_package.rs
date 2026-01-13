use auroka_utils::Host;
use auroka_utils_packages::{Package, PackageBuilder};
use rand::{Rng, distr::Alphanumeric};

pub fn build_and_expand_package(package: Package, suffix_len: usize) -> PackageBuilder {
  let suffix: String = rand::rng()
    .sample_iter(&Alphanumeric)
    .take(suffix_len)
    .map(char::from)
    .collect();

  let mut builder = PackageBuilder::try_new(&format!("macro_test_{}", suffix), package).unwrap();

  builder
    .expand_test_target(auroka_utils_packages::Environment::Default, &Host::host())
    .unwrap();

  builder
}
