use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
#[ignore]
async fn uses_safari_mobile_behavior() -> anyhow::Result<()> {
  with_server!(
    "/safari_mobile" => "<body><h1>Safari Mobile</h1></body>",
    |base_url| {
      // iOS Simulator often accesses host localhost just fine, or needs machine IP.
      // We'll use the generic URL and let the user handle network config if needed for now.
      with_page! { :SafariMobile
        base_url, "/safari_mobile", |page| {
          assert_has_text!(page.locator("h1"), "Safari Mobile");
        }
      }
    }
  )
}
