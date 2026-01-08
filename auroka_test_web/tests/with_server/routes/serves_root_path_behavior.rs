use auroka_assertions_web::assert_has_content;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_root_path_behavior() -> anyhow::Result<()> {
  with_server!(
    "/" => "Welcome Home",
    |base_url| {
      assert_has_content!(base_url, "/", "Welcome Home");
    }
  )
}
