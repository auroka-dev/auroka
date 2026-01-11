use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_opera_behavior() -> anyhow::Result<()> {
  with_server!(
    "/opera_mobile_emulation" => "<body><h1>Opera Mobile Emulation</h1></body>",
    |base_url| {
      with_page! { :Opera :Android :Emulator
        base_url, "/opera_mobile_emulation", |page| {
          assert_has_text!(page.locator("h1"), "Opera Mobile Emulation");
        }
      }
    }
  )
}
