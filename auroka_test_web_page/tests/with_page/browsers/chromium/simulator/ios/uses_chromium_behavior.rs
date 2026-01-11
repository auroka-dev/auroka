use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chromium_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chromium_ios" => "<body><h1>Chromium iOS</h1></body>",
    |base_url| {
      with_page! { :Chromium :IOS :Simulator
        base_url, "/chromium_ios", |page| {
          assert_has_text!(page.locator("h1"), "Chromium iOS");
        }
      }
    }
  )
}
