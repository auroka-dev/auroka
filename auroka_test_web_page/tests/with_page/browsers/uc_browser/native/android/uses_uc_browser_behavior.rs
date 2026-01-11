use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_uc_browser_behavior() -> anyhow::Result<()> {
  with_server!(
    "/uc_browser_android" => "<body><h1>UC Browser Android</h1></body>",
    |base_url| {
      with_page! { :UCBrowser :Android :Native
        base_url, "/uc_browser_android", |page| {
          assert_has_text!(page.locator("h1"), "UC Browser Android");
        }
      }
    }
  )
}
