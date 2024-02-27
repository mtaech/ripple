use axum::{middleware, Router};
use axum::response::{Html, Response};
use axum::routing::get;
use log::info;

use crate::layers::{cors_layer};

mod layers;
mod routers;


async fn hello()->Html<String>{
    Html("Hello World".to_string())
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // .nest("/api", routers::get_router())
        .route("/hello",get(hello))
        .fallback(hello)
        .layer(cors_layer());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("server run success at: http://127.0.0.1:8888")
}
