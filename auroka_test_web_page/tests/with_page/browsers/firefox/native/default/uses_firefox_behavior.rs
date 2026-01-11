use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_firefox_behavior() -> anyhow::Result<()> {
  with_server!(
    "/firefox" => "<body><h1>Firefox</h1></body>",
    |base_url| {
      with_page! { :Firefox
        base_url, "/firefox", |page| {
          assert_has_text!(page.locator("h1"), "Firefox");
        }
      }
    }
  )
}
