use auroka_assertions_web::assert_has_content_type;
use auroka_test_web::with_server;
use serde_json::json;

#[auroka::test]
async fn serves_json_content_type_behavior() -> anyhow::Result<()> {
  with_server!(
    "/api" => json!({"foo": "bar"}),
    |base_url| {
      assert_has_content_type!(base_url, "/api", "application/json");
    }
  )
}
