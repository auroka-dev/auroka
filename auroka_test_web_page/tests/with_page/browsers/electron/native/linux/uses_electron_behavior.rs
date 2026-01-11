use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_electron_behavior() -> anyhow::Result<()> {
  with_server!(
    "/electron" => "<body><h1>Electron</h1></body>",
    |base_url| {
      with_page! { :Electron :Native
        base_url, "/electron", |page| {
          assert_has_text!(page.locator("h1"), "Electron");
        }
      }
    }
  )
}
