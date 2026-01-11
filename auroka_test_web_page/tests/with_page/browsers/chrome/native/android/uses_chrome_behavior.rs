use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chrome_on_android_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome_android" => "<body><h1>ChromeMobile on Android</h1></body>",
    |base_url| {
      with_page! { :ChromeMobile :Android :Native
        base_url, "/chrome_android", |page| {
          assert_has_text!(page.locator("h1"), "ChromeMobile on Android");
        }
      }
    }
  )
}
