use auroka_assertions_web::assert_has_content;
use auroka_test_web::with_server;

#[tokio::test]
async fn serves_post_method_behavior() -> anyhow::Result<()> {
  with_server!(
    "/resource" => "Something else",
    "/resource" :Post => "Content",
    |base_url| {
      assert_has_content!(base_url, "/resource", :Post, "Content");
    }
  )
}
