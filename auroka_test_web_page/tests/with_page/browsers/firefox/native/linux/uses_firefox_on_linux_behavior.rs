use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_firefox_on_linux_behavior() -> anyhow::Result<()> {
  with_server!(
    "/firefox_linux" => "<body><h1>Firefox on Linux</h1></body>",
    |base_url| {
      with_page! { :Firefox :Linux :Native
        base_url, "/firefox_linux", |page| {
          assert_has_text!(page.locator("h1"), "Firefox on Linux");
        }
      }
    }
  )
}
