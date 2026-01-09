use auroka::web::page::{assert_has_text, with_page};
use auroka::web::with_server;

#[auroka::test]
async fn considers_page_with_base_url_and_path_behavior() -> anyhow::Result<()> {
  with_server!(
    "/check" => "<body>ok</body>",
    |base_url| {
      with_page! {
        base_url, "/check", |page| {
          assert_has_text!(page.locator("body"), "ok");
        }
      }
    }
  )
}
