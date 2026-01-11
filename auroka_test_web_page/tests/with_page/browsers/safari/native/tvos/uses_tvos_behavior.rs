use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_tvos_behavior() -> anyhow::Result<()> {
  with_server!(
    "/tv" => "<body><h1>Apple TV Emulation</h1></body>",
    |base_url| {
      with_page! { :Chrome :TvOS :Native
        base_url, "/tv", |page| {
          assert_has_text!(page.locator("h1"), "Apple TV Emulation");
        }
      }
    }
  )
}
