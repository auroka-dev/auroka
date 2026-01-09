use auroka::web::page::{assert_has_text, with_page};
use auroka_test_web::with_server;

#[auroka::test]
async fn considers_page_with_absolute_url_behavior() -> anyhow::Result<()> {
  with_server!(
    "/absolute" => "<body>absolute</body>",
    |base_url| {
      let full_url = format!("{}/absolute", base_url);
      with_page! {
        &full_url, |page| {
          assert_has_text!(page.locator("body"), "absolute");
        }
      }
    }
  )
}
