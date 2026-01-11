use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_safari_on_macos_behavior() -> anyhow::Result<()> {
  with_server!(
    "/safari_macos" => "<body><h1>Safari on Macos</h1></body>",
    |base_url| {
      with_page! { :Safari :MacOS
        base_url, "/safari_macos", |page| {
          assert_has_text!(page.locator("h1"), "Safari on Macos");
        }
      }
    }
  )
}
