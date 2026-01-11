use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_webkit_behavior() -> anyhow::Result<()> {
  with_server!(
    "/webkit_linux" => "<body><h1>WebKit Linux</h1></body>",
    |base_url| {
      with_page! { :WebKit :Linux :Native
        base_url, "/webkit_linux", |page| {
          assert_has_text!(page.locator("h1"), "WebKit Linux");
        }
      }
    }
  )
}
