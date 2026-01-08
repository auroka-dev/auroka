use auroka_assertions_web::assert_has_content_type;
use auroka_test_web::with_server;

#[tokio::test]
async fn serves_default_html_content_type_behavior() -> anyhow::Result<()> {
  with_server!(
    "/" => "<html><body>Hello</body></html>",
    |base_url| {
      assert_has_content_type!(base_url, "/", "text/html");
    }
  )
}
