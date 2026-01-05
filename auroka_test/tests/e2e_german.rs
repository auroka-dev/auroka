use auroka_test::auroka_test;
use auroka_test::browser::{expect, with_page};

#[auroka_test]
async fn loads_home_in_german() -> anyhow::Result<()> {
  with_page("/de", |page| async move {
    expect(page.locator("footer .socials"))
      .to_have_text("Folgen Sie uns in den sozialen Medien:")
      .await?;
    Ok(())
  })
  .await
}
