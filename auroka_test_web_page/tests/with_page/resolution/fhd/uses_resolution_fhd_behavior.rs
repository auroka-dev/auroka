use auroka_assertions_web_page::assert_has_content;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_resolution_fhd_behavior() -> anyhow::Result<()> {
  // 2. Using Clean DSL Syntax
  // Supports common presets like :HD, :FHD, :4K, :5K
  with_server! {
    "/" => "<h1>Example Domain</h1>",
    |url| {
      with_page! { :Chrome :FHD
        url, "/", |page| {
          assert_has_content!(page, "Example Domain");
        }
      }
    }
  }
}
