use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_edge_behavior() -> anyhow::Result<()> {
  with_server!(
    "/edge_android" => "<body><h1>EdgeMobile on Android</h1></body>",
    |base_url| {
      with_page! { :EdgeMobile :ChromeOS :Native
        base_url, "/edge_android", |page| {
          assert_has_text!(page.locator("h1"), "EdgeMobile on Android");
        }
      }
    }
  )
}
