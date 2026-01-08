use auroka_assertions_web::assert_has_content;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_put_method_behavior() -> anyhow::Result<()> {
  with_server!(
    "/resource" => "Content",
    |base_url| {
      assert_has_content!(base_url, "/resource", :Put, "Content");
    }
  )
}
