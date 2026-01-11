use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_safari_behavior() -> anyhow::Result<()> {
  with_server!(
    "/watch_os" => "<body><h1>WatchOS</h1></body>",
    |base_url| {
      with_page! { :Safari :WatchOS :Simulator
        base_url, "/watch_os", |page| {
          assert_has_text!(page.locator("h1"), "WatchOS");
        }
      }
    }
  )
}
