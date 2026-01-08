use auroka_assertions_web::assert_has_content;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_404_response_behavior() -> anyhow::Result<()> {
  with_server!(
      "/404" => (404, "Not Found Here"),
      |base_url| {
           assert_has_content!(base_url, "/404", "Not Found Here");
      }
  )
}
