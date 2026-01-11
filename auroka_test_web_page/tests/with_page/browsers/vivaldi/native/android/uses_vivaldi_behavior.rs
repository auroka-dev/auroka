use auroka_assertions_web_page::assert_has_text;
use auroka_test_web::with_server;
use auroka_test_web_page::with_page;

#[auroka::test]
async fn uses_vivaldi_behavior() -> anyhow::Result<()> {
  with_server!(
    "/vivaldi" => "<body><h1>Vivaldi</h1></body>",
    |base_url| {
      with_page! { :Vivaldi :Android :Native
        base_url, "/vivaldi", |page| {
          assert_has_text!(page.locator("h1"), "Vivaldi");
        }
      }
    }
  )
}
