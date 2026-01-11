use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_opera_behavior() -> anyhow::Result<()> {
  with_server!(
    "/opera" => "<body><h1>Opera</h1></body>",
    |base_url| {
      with_page! { :Opera :Windows :Native
        base_url, "/opera", |page| {
          assert_has_text!(page.locator("h1"), "Opera");
        }
      }
    }
  )
}
