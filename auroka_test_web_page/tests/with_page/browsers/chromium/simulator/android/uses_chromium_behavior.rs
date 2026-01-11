use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chromium_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chromium_android" => "<body><h1>Chromium Android</h1></body>",
    |base_url| {
      with_page! { :Chromium :Android :Simulator
        base_url, "/chromium_android", |page| {
          assert_has_text!(page.locator("h1"), "Chromium Android");
        }
      }
    }
  )
}
