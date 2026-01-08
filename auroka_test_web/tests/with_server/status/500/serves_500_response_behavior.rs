use auroka_assertions_web::assert_has_status;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_500_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/error" => (500, "Server Error"),
    |base_url| {
      assert_has_status!(base_url, "/error", 500);
    }
  )
}
