use auroka_assertions_web::assert_has_content_type;
use auroka_test_web::with_server;

#[tokio::test]
async fn serves_xml_content_type_behavior() -> anyhow::Result<()> {
  with_server!(
    "/data.xml" => (200, "application/xml", "<note><to>User</to><from>Server</from></note>"),
    |base_url| {
      assert_has_content_type!(base_url, "/data.xml","application/xml");
    }
  )
}
