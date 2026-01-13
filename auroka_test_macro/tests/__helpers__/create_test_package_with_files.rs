use auroka_utils::FileBuffer;
use auroka_utils_packages::{Dependency, Package};
use std::collections::HashMap;

pub fn create_test_package_with_files(invocation: &str, data: &HashMap<String, String>) -> Package {
  let mut package = Package::new("test");

  let mut auroka = Dependency::from_member("auroka", "auroka");
  auroka.set_default_features(false);
  package.add_dependency(auroka);

  package.add_file(FileBuffer::new("src/lib.rs", invocation));

  data.iter().for_each(|(file_name, content)| {
    package.add_file(FileBuffer::new(format!("src/__data__/{}", file_name), content.clone()));
  });

  package
}
