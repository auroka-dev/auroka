use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_edge_on_windows_behavior() -> anyhow::Result<()> {
  with_server!(
    "/edge_windows" => "<body><h1>Edge on Windows</h1></body>",
    |base_url| {
      with_page! { :Edge :Windows
        base_url, "/edge_windows", |page| {
          assert_has_text!(page.locator("h1"), "Edge on Windows");
        }
      }
    }
  )
}
