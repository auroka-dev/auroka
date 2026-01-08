use auroka_assertions_web::{assert_has_content, assert_has_status};
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_unknown_route_behavior() -> anyhow::Result<()> {
  with_server!(
    "/known" => "I exist",
    |base_url| {
      assert_has_status!(base_url, "/unknown", 404);
      assert_has_content!(base_url, "/unknown", "");
    }
  )
}
