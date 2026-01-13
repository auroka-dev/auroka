use crate::{Dependency, Package};
use auroka_utils::Workspace;
use auroka_utils_filesystem::Directory;

pub struct PackageGenerator {}

impl PackageGenerator {
  pub fn generate(root: &Directory, package: &Package) -> Result<(), std::io::Error> {
    Self::generate_cargo(root, package)?;
    Self::generate_files(root, package)
  }

  pub fn generate_cargo(root: &Directory, package: &Package) -> Result<(), std::io::Error> {
    let content = format!(
      "[package]
name = '{}'
version = '0.1.0'
edition = '2021'

[dependencies]
{}
[workspace]
",
      package.name(),
      Self::generate_dependencies(package),
    );

    root.write_file("Cargo.toml", &content)
  }

  pub fn generate_dependencies(package: &Package) -> String {
    let mut content = String::new();

    for dependency in package.dependencies() {
      match dependency {
        Dependency::Registry(registry) => {
          if registry.features().is_empty() {
            content.push_str(&format!("{} = '{}'\n", registry.name(), registry.version()));
          } else {
            content.push_str(&format!("{} = {{ version = '{}', features = {:?} }}\n", registry.name(), registry.version(), registry.features()));
          }
        }
        Dependency::Member(member) => {
          let mut options = Vec::new();
          options.push(format!("path='{}/'", Workspace::root_dir().join(member.member())));

          if !member.features().is_empty() {
            options.push(format!("features={:?}", member.features()));
          }

          if !member.default_features() {
            options.push("default-features=false".to_string());
          }

          content.push_str(&format!("{} = {{ {} }}\n", member.name(), options.join(", ")));
        }
      }
    }

    content
  }

  pub fn generate_files(root: &Directory, package: &Package) -> Result<(), std::io::Error> {
    for file in package.files() {
      root.write_file(file.name(), file.content())?;
    }

    Ok(())
  }
}
