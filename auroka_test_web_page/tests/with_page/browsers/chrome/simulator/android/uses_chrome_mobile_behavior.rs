use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_chrome_mobile_behavior() -> anyhow::Result<()> {
  with_server!(
    "/chrome_mobile_emulation" => "<body><h1>Chrome Mobile Emulation</h1></body>",
    |base_url| {
      with_page! { :Chrome :Android :Simulator
        base_url, "/chrome_mobile_emulation", |page| {
          assert_has_text!(page.locator("h1"), "Chrome Mobile Emulation");
        }
      }
    }
  )
}
