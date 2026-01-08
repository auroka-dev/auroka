use auroka_assertions_web::assert_has_status;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_201_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/created" => (201, "Created Resource"),
    |base_url| {
      assert_has_status!(base_url, "/created", 201);
    }
  )
}
