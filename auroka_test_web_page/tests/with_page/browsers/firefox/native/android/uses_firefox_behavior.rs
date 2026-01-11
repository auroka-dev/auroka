use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_firefox_behavior() -> anyhow::Result<()> {
  with_server!(
    "/firefox_android" => "<body><h1>Firefox on Android</h1></body>",
    |base_url| {
      with_page! { :Firefox :Android :Native
        base_url, "/firefox_android", |page| {
          assert_has_text!(page.locator("h1"), "Firefox on Android");
        }
      }
    }
  )
}
