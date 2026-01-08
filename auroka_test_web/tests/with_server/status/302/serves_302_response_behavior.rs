use auroka_assertions_web::assert_has_status;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_302_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/found" => (302, "Found"),
    |base_url| {
      assert_has_status!(base_url, "/found", 302);
    }
  )
}
