use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_vision_os_behavior() -> anyhow::Result<()> {
  with_server!(
    "/vision_os" => "<body><h1>VisionOS</h1></body>",
    |base_url| {
      with_page! { :Safari :VisionOS :Simulator
        base_url, "/vision_os", |page| {
          assert_has_text!(page.locator("h1"), "VisionOS");
        }
      }
    }
  )
}
