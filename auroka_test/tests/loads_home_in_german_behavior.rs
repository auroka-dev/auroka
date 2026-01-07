use auroka_test::auroka_test;
use auroka_test::web::{assert_has_text, with_page};

#[auroka_test]
async fn loads_home_in_german_behavior() -> anyhow::Result<()> {
  with_page!("/de", |page| {
    assert_has_text!(
      page.locator("footer .socials"),
      "Folgen Sie uns in den sozialen Medien:"
    );
  })
}
