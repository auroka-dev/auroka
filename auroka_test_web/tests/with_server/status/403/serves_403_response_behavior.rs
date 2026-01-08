use auroka_assertions_web::assert_has_status;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_403_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/forbidden" => (403, "Forbidden"),
    |base_url| {
      assert_has_status!(base_url, "/forbidden", 403);
    }
  )
}
