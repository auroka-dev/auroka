use super::generate_unique_folder_name;
use auroka_utils::FileBuffer;
use auroka_utils_packages::{Dependency, Package};

pub fn create_test_package(invocation: &str, data: &[(String, String)]) -> (String, Package) {
  let folder = generate_unique_folder_name();

  let mut package = Package::new("test");
  package.add_dependency(Dependency::from_member("auroka", "auroka"));

  let invocation_content = format!("use auroka::behavior;\n\n{}\n", invocation);

  package.add_file(FileBuffer::new("src/lib.rs", invocation_content));

  for (file_name, content) in data {
    package.add_file(FileBuffer::new(format!("src/__data__/{}", file_name), content.clone()));
  }

  (folder, package)
}
