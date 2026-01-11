use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_arc_behavior() -> anyhow::Result<()> {
  with_server!(
    "/arc" => "<body><h1>Arc Browser</h1></body>",
    |base_url| {
      with_page! { :Arc :MacOS :Native
        base_url, "/arc", |page| {
          assert_has_text!(page.locator("h1"), "Arc Browser");
        }
      }
    }
  )
}
