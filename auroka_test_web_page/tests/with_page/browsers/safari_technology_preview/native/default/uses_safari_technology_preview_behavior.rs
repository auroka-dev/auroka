use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_safari_technology_preview_behavior() -> anyhow::Result<()> {
  with_server!(
    "/safari_stp" => "<body><h1>Safari Technology Preview</h1></body>",
    |base_url| {
      with_page! { :SafariTechnologyPreview
        base_url, "/safari_stp", |page| {
          assert_has_text!(page.locator("h1"), "Safari Technology Preview");
        }
      }
    }
  )
}
