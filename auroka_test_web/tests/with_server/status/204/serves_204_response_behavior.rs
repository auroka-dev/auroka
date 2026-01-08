use auroka_assertions_web::{assert_has_content, assert_has_status};
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_204_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/no-content" => (204, ""),
    |base_url| {
      assert_has_status!(base_url, "/no-content", 204);
      assert_has_content!(base_url, "/no-content", "");
    }
  )
}
