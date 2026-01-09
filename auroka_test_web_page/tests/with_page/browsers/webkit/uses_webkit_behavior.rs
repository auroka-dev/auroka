use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
#[ignore]
async fn uses_webkit_behavior() -> anyhow::Result<()> {
  with_server!(
    "/webkit" => "<body><h1>WebKit</h1></body>",
    |base_url| {
      with_page! { :WebKit
        base_url, "/webkit", |page| {
          assert_has_text!(page.locator("h1"), "WebKit");
        }
      }
    }
  )
}
