use auroka_utils::FileBuffer;
use auroka_utils_packages::{Dependency, Package};
use rand::{Rng, distr::Alphanumeric};

pub fn create_test_package(invocation: &str, data: &[(String, String)]) -> (String, Package) {
  let rng = rand::rng();
  let suffix: String = rng
    .sample_iter(&Alphanumeric)
    .take(5)
    .map(char::from)
    .collect();

  let folder = format!("macro_auroka_test_tests_{}", suffix);

  let mut package = Package::new("test");
  package.add_dependency(Dependency::from_member("auroka", "auroka"));

  let invocation_content = format!(
    r#"use auroka::behavior;

{}
"#,
    invocation
  );

  package.add_file(FileBuffer::new("src/lib.rs", invocation_content));

  for (file_name, content) in data {
    package.add_file(FileBuffer::new(format!("src/__data__/{}", file_name), content.clone()));
  }

  (folder, package)
}
