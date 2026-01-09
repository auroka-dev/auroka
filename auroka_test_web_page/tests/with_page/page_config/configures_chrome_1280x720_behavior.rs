use auroka_assertions_web_page::assert_has_content;
use auroka_test_web::with_server;
use auroka_test_web_page::{Browser, PageConfig, with_page};

#[auroka::test]
async fn configures_chrome_1280x720_behavior() -> anyhow::Result<()> {
  // 1. Using Builder Pattern
  let config = PageConfig::new()
    .browser(Browser::Chrome)
    .viewport(1280, 720);

  with_server! {
    "/" => "<h1>Example Domain</h1>",
    |url| {
      with_page! { using config,
        url, "/", |page| {
          assert_has_content!(page, "Example Domain");
        }
      }
    }
  }
}
