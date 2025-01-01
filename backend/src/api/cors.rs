use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method
};
use tower_http::cors::{Any, CorsLayer};

pub fn configure_cors() -> CorsLayer {
    let origin = "http://localhost:3000"
        .parse::<HeaderValue>()
        .expect("Failed to parse CORS origin header value");

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
}
