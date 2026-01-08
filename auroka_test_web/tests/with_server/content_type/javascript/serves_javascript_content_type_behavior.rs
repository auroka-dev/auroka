use auroka_assertions_web::assert_has_content_type;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_javascript_content_type_behavior() -> anyhow::Result<()> {
  with_server!(
    "/script.js" => (200, "application/javascript", "console.log('hello world');"),
    |base_url| {
      assert_has_content_type!(base_url, "/script.js", "application/javascript");
    }
  )
}
