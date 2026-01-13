use serde_json::Value;

#[derive(Debug, Clone)]
pub struct MockResponse {
  pub status: u16,
  pub content_type: String,
  pub body: String,
}

impl Default for MockResponse {
  fn default() -> Self {
    Self { status: 200, content_type: "text/html".to_string(), body: "".to_string() }
  }
}

pub trait IntoMockResponse {
  fn into_mock_response(self) -> MockResponse;
}

impl IntoMockResponse for &str {
  fn into_mock_response(self) -> MockResponse {
    MockResponse { body: self.to_string(), ..Default::default() }
  }
}

impl IntoMockResponse for String {
  fn into_mock_response(self) -> MockResponse {
    MockResponse { body: self, ..Default::default() }
  }
}

impl IntoMockResponse for Value {
  fn into_mock_response(self) -> MockResponse {
    MockResponse { status: 200, content_type: "application/json".to_string(), body: self.to_string() }
  }
}

impl IntoMockResponse for (u16, &str) {
  fn into_mock_response(self) -> MockResponse {
    MockResponse { status: self.0, body: self.1.to_string(), ..Default::default() }
  }
}

impl IntoMockResponse for (u16, String) {
  fn into_mock_response(self) -> MockResponse {
    MockResponse { status: self.0, body: self.1, ..Default::default() }
  }
}

impl IntoMockResponse for (u16, &str, &str) {
  fn into_mock_response(self) -> MockResponse {
    MockResponse { status: self.0, content_type: self.1.to_string(), body: self.2.to_string() }
  }
}
