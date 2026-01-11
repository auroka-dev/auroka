use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_uc_browser_behavior() -> anyhow::Result<()> {
  with_server!(
    "/uc_browser_ios" => "<body><h1>UC Browser iOS</h1></body>",
    |base_url| {
      with_page! { :UCBrowser :IOS :Emulator
        base_url, "/uc_browser_ios", |page| {
          assert_has_text!(page.locator("h1"), "UC Browser iOS");
        }
      }
    }
  )
}
