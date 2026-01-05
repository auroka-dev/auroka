use anyhow::Error;
use tempfile::NamedTempFile;

pub struct Context {
  assembly_path: String,
  config: Option<Vec<NamedTempFile>>,
  error: Option<Error>,
  port: u16,
}

impl Context {
  pub fn new() -> Self {
    Self {
      assembly_path: String::new(),
      config: None,
      error: None,
      port: 0,
    }
  }

  pub fn assembly_path(&self) -> &str {
    &self.assembly_path
  }

  pub fn assembly_path_set(&mut self, assembly_path: String) {
    self.assembly_path = assembly_path;
  }

  pub fn config(&self) -> Option<&Vec<NamedTempFile>> {
    self.config.as_ref()
  }

  pub fn config_set(&mut self, config: Vec<NamedTempFile>) {
    self.config = Some(config);
  }

  pub fn error_set(&mut self, error: Error) {
    self.error = Some(error);
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub fn port_set(&mut self, port: u16) {
    self.port = port;
  }
}
