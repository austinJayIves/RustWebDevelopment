#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use axum::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Method,
    },
    routing::{delete, get, post, put},
    Extension, Router,
};
use stack_underflow::{
    models::store::Store,
    route::{
        answer::post_answer,
        question::{delete_question, get_question, get_questions, post_question, put_question},
    },
};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .json()
        .with_file(true)
        .with_span_events(FmtSpan::ACTIVE)
        .init();

    let store = Arc::new(Store::new());

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions/:id", get(get_question))
        .route("/questions", post(post_question))
        .route("/questions/:id", put(put_question))
        .route("/questions/:id", delete(delete_question))
        .route("/questions/:id/answers", post(post_answer))
        .layer(
            ServiceBuilder::new()
                .layer(cors_layer)
                .layer(Extension(Arc::clone(&store))),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
