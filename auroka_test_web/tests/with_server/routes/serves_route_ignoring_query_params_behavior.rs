use auroka_assertions_web::assert_has_content;
use auroka_test_web::with_server;

#[auroka::test]
async fn serves_route_ignoring_query_params_behavior() -> anyhow::Result<()> {
  with_server!(
    "/search" => "search results",
    |base_url| {
      assert_has_content!(base_url, "/search?q=test", "search results");
    }
  )
}
