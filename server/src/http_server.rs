use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};

use crate::ClientConnectionInfo;

use axum::http::{HeaderMap, HeaderName, HeaderValue, header};
use std::net::SocketAddr;

pub async fn run_http_server(http_addr: SocketAddr, client_connection_info: ClientConnectionInfo) {
    let listener = tokio::net::TcpListener::bind(http_addr)
        .await
        .expect("could not listen on HTTP address/port");

    // let client_connection_info = Arc::new(client_connection_info);
    let json_info = serde_json::to_string(&client_connection_info).unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/info",
            get(|| async move {
                let mut headers = HeaderMap::new();
                headers.insert(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_static("*"),
                );
                (headers, json_info.clone())
            }),
        );

    axum::serve(listener, app).await.unwrap();
}
