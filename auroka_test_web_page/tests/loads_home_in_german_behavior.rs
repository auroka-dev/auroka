use auroka::web::page::{assert_has_text, with_page};
use auroka::web::with_server;

#[auroka::test]
async fn loads_home_in_german_behavior() -> anyhow::Result<()> {
  with_server! {
    "/de" => "<html><body><footer><div class=\"socials\">Folgen Sie uns in den sozialen Medien:</div></footer></body></html>",
    |base_url| {
      with_page!(base_url, "/de", |page| {
        assert_has_text!(
          page.locator("footer .socials"),
          "Folgen Sie uns in den sozialen Medien:"
        );
      })
    }
  }
}
