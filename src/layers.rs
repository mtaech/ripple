use axum::extract::Request;
use axum::http::{header, Method};
use axum::middleware::Next;
use axum::response::IntoResponse;
use tower_http::cors::{AllowOrigin, CorsLayer};

///解除 CORS 限制
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![
            header::ACCEPT,
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
            header::ACCEPT_ENCODING,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
        ])
        // allow requests from any origin
        .allow_origin(AllowOrigin::any())
}