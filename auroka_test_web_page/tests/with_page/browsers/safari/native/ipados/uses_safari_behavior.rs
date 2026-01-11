use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_safari_behavior() -> anyhow::Result<()> {
  with_server!(
    "/safari_ipad" => "<body><h1>SafariMobile on IPad</h1></body>",
    |base_url| {
      with_page! { :Safari :IPadOS :Native
        base_url, "/safari_ipad", |page| {
          assert_has_text!(page.locator("h1"), "SafariMobile on IPad");
        }
      }
    }
  )
}
