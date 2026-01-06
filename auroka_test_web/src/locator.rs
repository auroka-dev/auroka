use crate::backends::Browser;
use std::sync::Arc;

#[derive(Clone)]
pub struct Locator {
  browser: Arc<dyn Browser>,
  selector: String,
}

impl Locator {
  pub fn new(browser: Arc<dyn Browser>, selector: &str) -> Self {
    Self {
      browser,
      selector: selector.to_string(),
    }
  }

  pub fn browser(&self) -> &Arc<dyn Browser> {
    &self.browser
  }

  pub fn selector(&self) -> &str {
    &self.selector
  }
}
