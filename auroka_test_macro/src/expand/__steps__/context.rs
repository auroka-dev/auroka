pub struct Context {
  error: Option<String>,
  expansion: Option<String>,
  invocation: Option<String>,
}

impl Context {
  pub fn new() -> Self {
    Self { error: None, expansion: None, invocation: None }
  }

  pub fn error(&self) -> &Option<String> {
    &self.error
  }

  pub fn error_set(&mut self, error: Option<String>) {
    self.error = error;
  }

  pub fn expansion(&self) -> &Option<String> {
    &self.expansion
  }

  pub fn expansion_set(&mut self, expansion: Option<String>) {
    self.expansion = expansion;
  }

  pub fn invocation(&self) -> &Option<String> {
    &self.invocation
  }

  pub fn invocation_set(&mut self, invocation: &str) {
    self.invocation = Some(invocation.to_string());
  }
}
