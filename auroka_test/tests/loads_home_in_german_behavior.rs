use auroka_test::auroka_test;
use auroka_test::web::{assert_has_text, with_page};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[auroka_test]
async fn loads_home_in_german_behavior() -> anyhow::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8787").await?;

  let server = tokio::spawn(async move {
    loop {
      let (mut socket, _) = listener.accept().await.unwrap();
      tokio::spawn(async move {
        let mut buf = [0; 1024];
        let _ = socket.read(&mut buf).await;
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><footer><div class=\"socials\">Folgen Sie uns in den sozialen Medien:</div></footer></body></html>";
        let _ = socket.write_all(response.as_bytes()).await;
      });
    }
  });

  let result = with_page!("/de", |page| {
    assert_has_text!(
      page.locator("footer .socials"),
      "Folgen Sie uns in den sozialen Medien:"
    );
  });

  server.abort();

  result
