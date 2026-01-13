use super::common::{buildkite_client, json_response, setup_mock_server};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use wiremock::matchers::{method, path};
use wiremock::{Mock, Request, ResponseTemplate};

#[tokio::test]
async fn test_packages_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/packages/organizations/my-org/registries/my-registry/packages/pkg-1"))
        .respond_with(json_response(r#"{
            "id": "pkg-1",
            "name": "my-package",
            "version": "1.0.0",
            "url": "https://api.buildkite.com/v2/packages/organizations/my-org/registries/my-registry/packages/pkg-1",
            "web_url": "https://buildkite.com/organizations/my-org/packages/registries/my-registry/pkg-1"
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let package = client
        .packages
        .get("my-org", "my-registry", "pkg-1")
        .await
        .unwrap();

    assert_eq!(package.id, Some("pkg-1".to_string()));
    assert_eq!(package.name, Some("my-package".to_string()));
    assert_eq!(package.version, Some("1.0.0".to_string()));
}

#[tokio::test]
async fn test_packages_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/packages/organizations/my-org/registries/my-registry/packages/pkg-1",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .packages
        .delete("my-org", "my-registry", "pkg-1")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_packages_request_presigned_upload() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/packages/organizations/my-org/registries/my-registry/packages/upload"))
        .respond_with(json_response(r#"{
            "uri": "s3://fake-s3-bucket/fake-s3-path",
            "form": {
                "file_input": "file",
                "method": "POST",
                "url": "https://s3.example.com/bucket",
                "data": {
                    "key": "uploads/${filename}",
                    "acl": "private",
                    "policy": "base64policy",
                    "x-amz-credential": "AKIAS000000000000000/20241007/us-east-1/s3/aws4_request",
                    "x-amz-algorithm": "AWS4-HMAC-SHA256",
                    "x-amz-date": "20241007T031838Z",
                    "x-amz-signature": "f6d24942026ffe7ec32b5f57beb46a2679b7a74a87673e1614b92c15ee2661f2"
                }
            }
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let presigned = client
        .packages
        .request_presigned_upload("my-org", "my-registry")
        .await
        .unwrap();

    assert_eq!(presigned.uri, "s3://fake-s3-bucket/fake-s3-path");
    assert_eq!(presigned.form.file_input, "file");
    assert_eq!(presigned.form.method, "POST");
    assert_eq!(presigned.form.url, "https://s3.example.com/bucket");
    assert!(presigned.form.data.contains_key("key"));
    assert!(presigned.form.data.contains_key("policy"));
}

#[tokio::test]
async fn test_encoding_file() {
    let s3_mock_server = setup_mock_server().await;
    let captured_body: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    let captured_body_clone = captured_body.clone();

    Mock::given(method("POST"))
        .respond_with(move |req: &Request| {
            let body = req.body.clone();
            let captured = captured_body_clone.clone();
            tokio::spawn(async move {
                let mut guard = captured.lock().await;
                *guard = body;
            });
            ResponseTemplate::new(204)
        })
        .mount(&s3_mock_server)
        .await;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    {
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();
        file.write_all(b"hello world").await.unwrap();
        file.flush().await.unwrap();
    }

    let presigned = rust_buildkite::PackagePresignedUpload {
        uri: "s3://test-bucket/test-path".to_string(),
        form: rust_buildkite::PackagePresignedUploadForm {
            file_input: "file".to_string(),
            method: "POST".to_string(),
            url: s3_mock_server.uri(),
            data: [("key".to_string(), "uploads/${filename}".to_string())]
                .into_iter()
                .collect(),
        },
    };

    let file = tokio::fs::File::open(&file_path).await.unwrap();
    let mock_server = setup_mock_server().await;
    let client = buildkite_client(&mock_server);

    let result = client
        .packages
        .perform_upload(&presigned, file, "test.txt")
        .await;
    assert!(result.is_ok());

    let s3_url = result.unwrap();
    assert!(s3_url.contains("uploads/test.txt"));

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let body = captured_body.lock().await;
    let body_str = String::from_utf8_lossy(&body);
    assert!(
        body_str.contains("hello world"),
        "file contents = {:?}, want \"hello world\"",
        body_str
    );
    assert!(
        body_str.contains("test.txt"),
        "filename not found in multipart body"
    );
    assert!(
        body_str.contains("Content-Disposition"),
        "body should be multipart encoded"
    );
}

#[tokio::test]
async fn test_encoding_form_fields() {
    let s3_mock_server = setup_mock_server().await;
    let captured_body: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    let captured_body_clone = captured_body.clone();

    Mock::given(method("POST"))
        .respond_with(move |req: &Request| {
            let body = req.body.clone();
            let captured = captured_body_clone.clone();
            tokio::spawn(async move {
                let mut guard = captured.lock().await;
                *guard = body;
            });
            ResponseTemplate::new(204)
        })
        .mount(&s3_mock_server)
        .await;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    {
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();
        file.write_all(b"content").await.unwrap();
        file.flush().await.unwrap();
    }

    let presigned = rust_buildkite::PackagePresignedUpload {
        uri: "s3://test-bucket/test-path".to_string(),
        form: rust_buildkite::PackagePresignedUploadForm {
            file_input: "file".to_string(),
            method: "POST".to_string(),
            url: s3_mock_server.uri(),
            data: [
                ("key".to_string(), "uploads/${filename}".to_string()),
                ("mountain".to_string(), "cotopaxi".to_string()),
                ("city".to_string(), "guayaquil".to_string()),
            ]
            .into_iter()
            .collect(),
        },
    };

    let file = tokio::fs::File::open(&file_path).await.unwrap();
    let mock_server = setup_mock_server().await;
    let client = buildkite_client(&mock_server);

    let result = client
        .packages
        .perform_upload(&presigned, file, "test.txt")
        .await;
    assert!(result.is_ok());

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let body = captured_body.lock().await;
    let body_str = String::from_utf8_lossy(&body);
    assert!(
        body_str.contains("cotopaxi"),
        "form.Value[\"mountain\"] not found"
    );
    assert!(
        body_str.contains("guayaquil"),
        "form.Value[\"city\"] not found"
    );
}

#[tokio::test]
async fn test_finalize_upload() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/packages/organizations/my-org/registries/my-registry/packages",
        ))
        .respond_with(json_response(
            r#"{
            "id": "pkg-created",
            "name": "my-package",
            "version": "1.0.0"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let package = client
        .packages
        .finalize_upload(
            "my-org",
            "my-registry",
            "https://s3.example.com/bucket/uploads/my-package.tar.gz",
        )
        .await
        .unwrap();

    assert_eq!(package.id, Some("pkg-created".to_string()));
    assert_eq!(package.name, Some("my-package".to_string()));
}

#[tokio::test]
async fn test_create_from_file() {
    let bk_mock_server = setup_mock_server().await;
    let s3_mock_server = setup_mock_server().await;
    let s3_url = s3_mock_server.uri();

    Mock::given(method("POST"))
        .and(path(
            "/v2/packages/organizations/my-org/registries/my-registry/packages/upload",
        ))
        .respond_with(json_response(&format!(
            r#"{{
            "uri": "s3://test-bucket/test-path",
            "form": {{
                "file_input": "file",
                "method": "POST",
                "url": "{}",
                "data": {{ "key": "uploads/${{filename}}", "acl": "private" }}
            }}
        }}"#,
            s3_url
        )))
        .mount(&bk_mock_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&s3_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/packages/organizations/my-org/registries/my-registry/packages",
        ))
        .respond_with(json_response(
            r#"{ "id": "pkg-final", "name": "final-package", "version": "2.0.0" }"#,
        ))
        .mount(&bk_mock_server)
        .await;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("package.tar.gz");
    {
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();
        file.write_all(b"package content").await.unwrap();
        file.flush().await.unwrap();
    }

    let client = buildkite_client(&bk_mock_server);
    let package = client
        .packages
        .create_from_file("my-org", "my-registry", &file_path)
        .await
        .unwrap();

    assert_eq!(package.id, Some("pkg-final".to_string()));
    assert_eq!(package.name, Some("final-package".to_string()));
    assert_eq!(package.version, Some("2.0.0".to_string()));
}
