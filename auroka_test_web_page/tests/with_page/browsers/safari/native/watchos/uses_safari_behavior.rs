use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_safari_on_watchos_behavior() -> anyhow::Result<()> {
  with_server!(
    "/safari_watchos" => "<body><h1>SafariMobile on WatchOS</h1></body>",
    |base_url| {
      with_page! { :Safari :WatchOS :Native
        base_url, "/safari_watchos", |page| {
          assert_has_text!(page.locator("h1"), "SafariMobile on WatchOS");
        }
      }
    }
  )
}
