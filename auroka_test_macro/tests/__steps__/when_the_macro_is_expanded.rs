use super::Context;
use auroka_utils::{FileBuffer, Host, Workspace};
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

  // 1. Create a mock `auroka_test` package that is lighter than the real one
  // This avoids compiling heavy dependencies like `chromiumoxide` during tests
  // and prevents potential deadlocks with the main workspace lock.
  let mut mock_package = Package::new("auroka_test");
  mock_package.add_dependency(Dependency::from_member("auroka_test_macro", "auroka_test_macro"));
  mock_package.add_dependency(Dependency::from_member("auroka_test_registry", "auroka_test_registry"));

  mock_package.add_file(FileBuffer::new(
    "src/lib.rs",
    r#"
    pub use auroka_test_macro::auroka_test;
    pub use auroka_test_registry::{inventory, Test, TestReturn};
    "#,
  ));

  let mut mock_builder = PackageBuilder::try_new(&folder, mock_package).unwrap();
  mock_builder.generate().unwrap();
  // Prevent mock_builder from cleaning up the parent directory when dropped,
  // because package_builder will do it (and they share the parent).
  std::mem::forget(mock_builder);

  // 2. Create the test package that uses the mock
  let mut package = Package::new("test");

  let mock_path = Workspace::target_dir().join(&folder).join("auroka_test");

  // We point "auroka_test" to our mock package
  package.add_dependency(Dependency::from_member("auroka_test", mock_path.path().to_str().unwrap()));

  let invocation = format!(
    r#"use auroka_test::auroka_test;

{}
"#,
    context.invocation().as_ref().unwrap()
  );

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
