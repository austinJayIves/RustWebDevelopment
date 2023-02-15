use std::net::SocketAddr;

use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/:name", get(hello_route));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_route(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}
