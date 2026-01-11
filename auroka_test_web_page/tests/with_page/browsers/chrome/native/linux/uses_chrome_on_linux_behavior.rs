use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chrome_on_linux_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome_linux" => "<body><h1>Chrome on Linux</h1></body>",
    |base_url| {
      with_page! { :Chrome :Linux
        base_url, "/chrome_linux", |page| {
          assert_has_text!(page.locator("h1"), "Chrome on Linux");
        }
      }
    }
  )
}
