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

  pub fn set_features(&mut self, features: Vec<String>) {
    match self {
      Dependency::Member(member) => member.set_features(features),
      Dependency::Registry(_) => unimplemented!("Registry features update not implemented yet"),
    }
  }

  pub fn set_default_features(&mut self, default_features: bool) {
    match self {
      Dependency::Member(member) => member.set_default_features(default_features),
      Dependency::Registry(_) => unimplemented!("Registry default features update not implemented yet"),
    }
  }
}
