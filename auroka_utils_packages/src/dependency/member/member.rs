#[derive(Debug)]
pub struct Member {
  name: String,
  member: String,
  features: Vec<String>,
  default_features: bool,
}

impl Member {
  pub fn new(name: &str, member: &str) -> Self {
    Member { name: name.to_string(), member: member.to_string(), features: Vec::new(), default_features: true }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn member(&self) -> &str {
    &self.member
  }

  pub fn features(&self) -> &[String] {
    &self.features
  }

  pub fn default_features(&self) -> bool {
    self.default_features
  }

  pub fn set_features(&mut self, features: Vec<String>) {
    self.features = features;
  }

  pub fn set_default_features(&mut self, default_features: bool) {
    self.default_features = default_features;
  }
}
