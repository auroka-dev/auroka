use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
#[ignore]
async fn uses_firefox_mobile_behavior() -> anyhow::Result<()> {
  with_server!(
    "/firefox_mobile" => "<body><h1>Firefox Mobile</h1></body>",
    |base_url| {
      with_page! { :FirefoxMobile
        base_url, "/firefox_mobile", |page| {
          assert_has_text!(page.locator("h1"), "Firefox Mobile");
        }
      }
    }
  )
}
