use auroka_assertions_web_page::assert_has_content;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_resolution_1024x768_behavior() -> anyhow::Result<()> {
  // Custom resolution (requires spaces around 'x' currently due to tokenization)
  with_server! {
    "/" => "<h1>Example Domain</h1>",
    |url| {
      with_page! { :Chrome :1024 x 768
        url, "/", |page| {
          assert_has_content!(page, "Example Domain");
        }
      }
    }
  }
}
