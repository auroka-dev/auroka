use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_google_chrome_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome" => "<body><h1>Chrome</h1></body>",
    |base_url| {
      with_page! { :Chrome :ChromeOS :Native
         base_url, "/chrome", |page| {
            assert_has_text!(page.locator("h1"), "Chrome");
         }
      }
    }
  )
}
