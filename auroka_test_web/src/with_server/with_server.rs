use crate::with_server::response::MockResponse;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn status_reason(status: u16) -> &'static str {
  match status {
    200 => "OK",
    201 => "Created",
    204 => "No Content",
    301 => "Moved Permanently",
    302 => "Found",
    400 => "Bad Request",
    401 => "Unauthorized",
    403 => "Forbidden",
    404 => "Not Found",
    500 => "Internal Server Error",
    _ => "Unknown",
  }
}

pub trait IntoTestResult {
  fn into_test_result(self) -> anyhow::Result<()>;
}

impl IntoTestResult for () {
  fn into_test_result(self) -> anyhow::Result<()> {
    Ok(())
  }
}

impl<E> IntoTestResult for Result<(), E>
where
  E: Into<anyhow::Error>,
{
  fn into_test_result(self) -> anyhow::Result<()> {
    self.map_err(|e| e.into())
  }
}

#[doc(hidden)]
pub async fn with_server_internal<F, Fut, R>(
  routes: Vec<(String, String, MockResponse)>,
  test_fn: F,
) -> anyhow::Result<()>
where
  F: FnOnce(String) -> Fut,
  Fut: Future<Output = R>,
  R: IntoTestResult,
{
  let listener = TcpListener::bind("127.0.0.1:0").await?;
  let port = listener.local_addr()?.port();
  let base_url = format!("http://127.0.0.1:{}", port);

  let route_map: HashMap<(String, String), MockResponse> = routes
    .into_iter()
    .map(|(method, path, resp)| ((method, path), resp))
    .collect();
  let route_map = Arc::new(route_map);

  let server_handle = tokio::spawn(async move {
    loop {
      let (mut socket, _) = match listener.accept().await {
        Ok(s) => s,
        Err(_) => continue,
      };
      let route_map = route_map.clone();
      tokio::spawn(async move {
        let mut buf = [0; 1024];
        let n = match socket.read(&mut buf).await {
          Ok(n) if n == 0 => return,
          Ok(n) => n,
          Err(_) => return,
        };

        let request = String::from_utf8_lossy(&buf[..n]);
        let mut parts = request.split_whitespace();
        if let Some(method) = parts.next() {
          if let Some(raw_path) = parts.next() {
            let path = raw_path.split('?').next().unwrap_or(raw_path);
            if let Some(response) = route_map.get(&(method.to_string(), path.to_string())) {
              let reason = status_reason(response.status);
              let response_str = format!(
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                response.status,
                reason,
                response.content_type,
                response.body.len(),
                response.body
              );
              let _ = socket.write_all(response_str.as_bytes()).await;
              return;
            }
          }
        }

        let _ = socket.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").await;
      });
    }
  });

  let result = test_fn(base_url).await.into_test_result();

  server_handle.abort();

  result
}

#[macro_export]
macro_rules! with_server {
  // 1. First route is Method-based: "Path" :Method => Content
  ( $path:literal : $method:ident => $content:expr, $($rest:tt)+ ) => {
    with_server!(@internal_init vec![ (stringify!($method).to_uppercase(), $path.to_string(), { use $crate::IntoMockResponse; $content.into_mock_response() }) ], $($rest)+)
  };

  // 2. First route is Default (GET): "Path" => Content
  ( $path:literal => $content:expr, $($rest:tt)+ ) => {
    with_server!(@internal_init vec![ ("GET".to_string(), $path.to_string(), { use $crate::IntoMockResponse; $content.into_mock_response() }) ], $($rest)+)
  };

  // 3. Stop condition: Closure
  (@internal_init $vec:expr, |$base_url:ident| $body:block ) => {
    $crate::with_server_internal($vec, |$base_url| async move {
      use $crate::IntoTestResult;
      let res = { $body };
      res.into_test_result()
    })
    .await
  };

  // 4. Recursive: Next route is Method-based
  (@internal_init $vec:expr, $path:literal : $method:ident => $content:expr, $($rest:tt)+ ) => {
    with_server!(@internal_init {
      let mut v = $vec;
      v.push((stringify!($method).to_uppercase(), $path.to_string(), { use $crate::IntoMockResponse; $content.into_mock_response() }));
      v
    }, $($rest)+)
  };

  // 5. Recursive: Next route is Default (GET)
  (@internal_init $vec:expr, $path:literal => $content:expr, $($rest:tt)+ ) => {
    with_server!(@internal_init {
      let mut v = $vec;
      v.push(("GET".to_string(), $path.to_string(), { use $crate::IntoMockResponse; $content.into_mock_response() }));
      v
    }, $($rest)+)
  };
}
