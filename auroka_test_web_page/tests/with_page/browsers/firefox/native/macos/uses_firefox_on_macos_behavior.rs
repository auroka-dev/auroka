use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_firefox_on_macos_behavior() -> anyhow::Result<()> {
  with_server!(
    "/firefox_macos" => "<body><h1>Firefox on Macos</h1></body>",
    |base_url| {
      with_page! { :Firefox :MacOS
        base_url, "/firefox_macos", |page| {
          assert_has_text!(page.locator("h1"), "Firefox on Macos");
        }
      }
    }
  )
}
