use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer, http::{Method, StatusCode}, routing::get, BoxError, Router
};
use rust_ddd::{database::postgres::PostgresDatabase, settings::settings::Settings};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, trace::TraceLayer};
use tracing::log::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Settings::new();

    let db = PostgresDatabase::new(&setting.database);

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET, 
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE
                ])
                .allow_origin(Any)
        )
        .layer(RequestBodyLimitLayer::new(setting.server.body_limit.try_into().unwrap()))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(setting.server.timeout.try_into().unwrap())))
        )
        .fallback(|| async { "Not Found" });

    let uri = format!("0.0.0.0:{}", setting.server.port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();

    info!("🦀 Server is starting on: :{}", setting.server.port);

    axum::serve(listener, app).await.unwrap();
}
