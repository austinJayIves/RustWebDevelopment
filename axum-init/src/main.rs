use std::net::SocketAddr;

use axum::{
    routing::{
        get 
    },
    Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "Hello World"
}
