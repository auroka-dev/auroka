use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_brave_behavior() -> anyhow::Result<()> {
  with_server!(
    "/brave_ios" => "<body><h1>BraveMobile on iOS</h1></body>",
    |base_url| {
      with_page! { :BraveMobile :IOS :Native
        base_url, "/brave_ios", |page| {
          assert_has_text!(page.locator("h1"), "BraveMobile on iOS");
        }
      }
    }
  )
}
