use crate::Locator;
use anyhow::{Result, anyhow};

pub struct Expect {
  locator: Locator,
}

pub fn expect(locator: Locator) -> Expect {
  Expect { locator }
}

impl Expect {
  pub async fn to_have_text(&self, expected: &str) -> Result<()> {
    let element = self
      .locator
      .browser()
      .find_element(self.locator.selector())
      .await?;
    let text = element
      .inner_text()
      .await?
      .ok_or_else(|| anyhow!("Element has no text"))?;

    if text.contains(expected) {
      Ok(())
    } else {
      Err(anyhow!("Expected text '{}', found '{}'", expected, text))
    }
  }
}
