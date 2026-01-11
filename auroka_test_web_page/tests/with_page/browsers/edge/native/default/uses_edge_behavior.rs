use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_edge_behavior() -> anyhow::Result<()> {
  with_server!(
    "/edge" => "<body><h1>Edge</h1></body>",
    |base_url| {
      with_page! { :Edge
        base_url, "/edge", |page| {
          assert_has_text!(page.locator("h1"), "Edge");
        }
      }
    }
  )
}
