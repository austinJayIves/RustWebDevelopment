use std::net::SocketAddr;
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use axum::{
    extract::Path,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use stack_underflow::models::{
    error::{AppError, QuestionRepoError},
    question::{Question, QuestionId, Tag},
};

#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route("/hello/:name", get(hello_route))
        .route("/questions", get(get_questions))
        .route("/questions/:id", get(get_question))
        .layer(ServiceBuilder::new().layer(cors_layer));

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

async fn get_questions() -> impl IntoResponse {
    let question = Question::new(
        QuestionId::from_str("ABC").expect("Unable to parse question ID"),
        "What is the meaning of life, the universe, and everything".to_owned(),
        "I heard it was '42', but that doesn't sound right".to_owned(),
        Some(vec![Tag::from_str("Philosophy").expect("Valid tag")]),
    );

    (StatusCode::OK, Json(question))
}

async fn get_question(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    if id == "XYZ" {
        return Err(QuestionRepoError::InvalidId.into());
    }

    let question = Question::new(
        QuestionId::from_str(&id).map_err(|_| QuestionRepoError::InvalidId)?,
        "What is the meaning of life, the universe, and everything".to_owned(),
        "I heard it was '42', but that doesn't sound right".to_owned(),
        Some(vec![Tag::from_str("Philosophy").expect("Valid tag")]),
    );

    Ok((StatusCode::OK, Json(question)))
}
