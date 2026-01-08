use auroka_assertions_web::assert_has_status;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_301_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/moved" => (301, "Moved"),
    |base_url| {
      assert_has_status!(base_url, "/moved", 301);
    }
  )
}
