use auroka_assertions_web::assert_has_content;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_multiple_routes_behavior() -> anyhow::Result<()> {
  with_server!(
      "/a" => "content-a",
      "/b" => "content-b",
      |base_url| {
          assert_has_content!(base_url, "/a", "content-a");
          assert_has_content!(base_url, "/b", "content-b");
      }
  )
}
