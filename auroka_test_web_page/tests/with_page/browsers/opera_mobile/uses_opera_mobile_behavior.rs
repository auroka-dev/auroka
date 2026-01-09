use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
#[ignore]
async fn uses_opera_mobile_behavior() -> anyhow::Result<()> {
  with_server!(
    "/opera_mobile" => "<body><h1>Opera Mobile</h1></body>",
    |base_url| {
      with_page! { :OperaMobile
        base_url, "/opera_mobile", |page| {
          assert_has_text!(page.locator("h1"), "Opera Mobile");
        }
      }
    }
  )
}
