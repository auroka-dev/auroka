use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chrome_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome_ios" => "<body><h1>ChromeMobile on iOS</h1></body>",
    |base_url| {
      with_page! { :ChromeMobile :IOS :Native
        base_url, "/chrome_ios", |page| {
          assert_has_text!(page.locator("h1"), "ChromeMobile on iOS");
        }
      }
    }
  )
}
