use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chrome_on_macos_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome_macos" => "<body><h1>Chrome on MacOS</h1></body>",
    |base_url| {
      with_page! { :Chrome :MacOS
        base_url, "/chrome_macos", |page| {
          assert_has_text!(page.locator("h1"), "Chrome on MacOS");
        }
      }
    }
  )
}
