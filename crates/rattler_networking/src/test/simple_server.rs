use std::{future::IntoFuture, io::Write, sync::Arc};

use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{
    authentication_storage::backends::netrc::NetRcStorage, AuthenticationMiddleware,
    AuthenticationStorage,
};

async fn health_checker_handler() -> impl IntoResponse {
    return "test";
}

async fn basic_http_auth(headers: HeaderMap) -> impl IntoResponse {
    println!("{:?}", headers);
    match headers.get("Authorization") {
        Some(auth) => {
            let auth = auth.to_str().unwrap();
            // user:password = test:test
            if auth == "Basic dGVzdDp0ZXN0" {
                return StatusCode::OK;
            }
        }
        None => {}
    }
    return StatusCode::UNAUTHORIZED;
}

async fn bearer_token_auth(headers: HeaderMap) -> impl IntoResponse {
    match headers.get("Authorization") {
        Some(auth) => {
            let auth = auth.to_str().unwrap();
            if auth == "Bearer test" {
                return StatusCode::OK;
            }
        }
        None => {}
    }
    return StatusCode::UNAUTHORIZED;
}

async fn token_auth(Path(token): Path<String>) -> impl IntoResponse {
    if token == "test" {
        return StatusCode::OK;
    }
    return StatusCode::UNAUTHORIZED;
}

struct SimpleServer {
    shutdown_sender: Option<oneshot::Sender<()>>,
    address: String,
}

async fn spawn() -> SimpleServer {
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/basic_http_auth", get(basic_http_auth))
        .route("/api/bearer_token_auth", get(bearer_token_auth))
        .route("/api/:token/token_auth", get(token_auth));

    let address = "0.0.0.0:1234".to_string();
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let (tx, rx) = oneshot::channel();

    let server = axum::serve(listener, app)
        .with_graceful_shutdown(async {
            rx.await.ok();
        })
        .into_future();

    tokio::spawn(server);

    println!("🚀 Server started successfully");

    SimpleServer {
        shutdown_sender: Some(tx),
        address,
    }
}

impl SimpleServer {
    pub fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.address, path)
    }
}

impl Drop for SimpleServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_sender.take() {
            let _ = tx.send(());
        }
    }
}

// #[tokio::test]
async fn test_server() {
    let server = spawn().await;

    let client = reqwest::Client::new();
    let response = client
        .get(server.url("/api/basic_http_auth"))
        .basic_auth("test", Some("test"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let response = client
        .get(server.url("/api/basic_http_auth"))
        .basic_auth("test", Some("false"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);

    let response = client
        .get(server.url("/api/bearer_token_auth"))
        .bearer_auth("test")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let response = client
        .get(server.url("/api/bearer_token_auth"))
        .bearer_auth("false")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);

    let response = client
        .get(server.url("/api/test/token_auth"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let response = client
        .get(server.url("/api/false/token_auth"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

// test netrc authenticated client
#[tokio::test]
async fn test_netrc() {
    let server = spawn().await;

    // write netrc file
    let mut netrc_file = tempfile::NamedTempFile::new().unwrap();
    netrc_file
        .write_all(b"machine 0.0.0.0\nlogin test\npassword test")
        .unwrap();

    let mut storage = AuthenticationStorage::new();
    storage.add_backend(Arc::from(
        NetRcStorage::from_path(netrc_file.path()).unwrap(),
    ));
    let middleware = AuthenticationMiddleware::new(storage);

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::default())
        .with_arc(Arc::new(middleware))
        .build();

    let response = client
        .get(server.url("/api/basic_http_auth"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}
