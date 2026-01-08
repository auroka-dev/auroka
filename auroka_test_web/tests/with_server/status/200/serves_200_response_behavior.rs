use auroka_assertions_web::assert_has_status;
use auroka_test_web::with_server;

#[tokio::test]
async fn serves_200_response_behavior() -> anyhow::Result<()> {
  with_server!(
    "/text" => (200, "text/plain", "Simple Text"),
    |base_url| {
      assert_has_status!(base_url, "/text", 200);
    }
  )
}
