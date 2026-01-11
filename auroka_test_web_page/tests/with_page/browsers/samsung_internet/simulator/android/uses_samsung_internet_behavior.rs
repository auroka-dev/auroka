use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_samsung_internet_behavior() -> anyhow::Result<()> {
  with_server!(
    "/samsung_internet_android" => "<body><h1>Samsung Internet Android</h1></body>",
    |base_url| {
      with_page! { :SamsungInternet :Android :Simulator
        base_url, "/samsung_internet_android", |page| {
          assert_has_text!(page.locator("h1"), "Samsung Internet Android");
        }
      }
    }
  )
}
