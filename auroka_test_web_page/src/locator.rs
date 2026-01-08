use crate::backends::Backend;
use std::sync::Arc;

#[derive(Clone)]
pub struct Locator {
  backend: Arc<dyn Backend>,
  selector: String,
}

impl Locator {
  pub fn new(backend: Arc<dyn Backend>, selector: &str) -> Self {
    Self {
      backend,
      selector: selector.to_string(),
    }
  }

  pub fn backend(&self) -> &Arc<dyn Backend> {
    &self.backend
  }

  pub fn selector(&self) -> &str {
    &self.selector
  }
}
