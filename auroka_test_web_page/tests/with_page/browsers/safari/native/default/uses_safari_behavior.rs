use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_safari_behavior() -> anyhow::Result<()> {
  with_server!(
    "/safari" => "<body><h1>Safari</h1></body>",
    |base_url| {
      with_page! { :Safari
        base_url, "/safari", |page| {
          assert_has_text!(page.locator("h1"), "Safari");
        }
      }
    }
  )
}
