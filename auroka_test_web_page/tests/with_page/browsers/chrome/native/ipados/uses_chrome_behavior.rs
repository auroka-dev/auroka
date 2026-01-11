use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chrome_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome_ipados" => "<body><h1>ChromeMobile on iPadOS</h1></body>",
    |base_url| {
      with_page! { :ChromeMobile :IPadOS :Native
        base_url, "/chrome_ipados", |page| {
          assert_has_text!(page.locator("h1"), "ChromeMobile on iPadOS");
        }
      }
    }
  )
}
