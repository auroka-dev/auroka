#[derive(Debug)]
pub struct Registry {
  name: String,
  features: Vec<String>,
  version: String,
}

impl Registry {
  pub fn new(name: &str, version: &str, features: &[&str]) -> Self {
    Registry {
      name: name.to_string(),
      version: version.to_string(),
      features: features.iter().map(|feature| feature.to_string()).collect(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn features(&self) -> &[String] {
    &self.features
  }

  pub fn version(&self) -> &str {
    &self.version
  }
}
