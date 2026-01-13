use super::{build_and_expand_package, create_test_package_with_files};
use auroka_utils_packages::PackageBuilder;
use std::collections::HashMap;

pub fn expand_in_temporary_package(invocation: &str, data: &HashMap<String, String>) -> PackageBuilder {
  let package = create_test_package_with_files(invocation, data);
  build_and_expand_package(package, 5)
}
