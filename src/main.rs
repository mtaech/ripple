use axum::response::{Html, Response};
use axum::routing::get;
use axum::{Extension, middleware, Router};
use dotenvy::dotenv;
use log::info;

use crate::layers::cors_layer;
use crate::setup::db::{get_db_conn, init_db};
use crate::setup::logger::init_logger;

mod api;
mod entity;
mod layers;
mod routers;
mod setup;

async fn hello() -> Html<String> {
    Html("Hello World".to_string())
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    init_logger();
    let db_conn = init_db().await;

    // build our application with a route
    let app = Router::new()
        // .nest("/api", routers::get_router())
        .route("/hello", get(hello))
        .fallback(hello)
        .layer(Extension(db_conn))
        .layer(cors_layer());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("server run success at: http://127.0.0.1:8888")
}
