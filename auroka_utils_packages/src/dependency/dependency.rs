use super::{member::Member, registry::Registry};

#[derive(Debug)]
pub enum Dependency {
  Member(Member),
  Registry(Registry),
}

impl Dependency {
  pub fn from_member(name: &str, member: &str) -> Self {
    Dependency::Member(Member::new(name, member))
  }

  pub fn from_registry(name: &str, version: &str, features: &[&str]) -> Self {
    Dependency::Registry(Registry::new(name, version, features))
  }
}
