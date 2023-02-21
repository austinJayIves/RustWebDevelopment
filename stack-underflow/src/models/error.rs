use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AppError {
    QuestionRepo(QuestionRepoError),
}

pub enum QuestionRepoError {
    InvalidId,
}

impl From<QuestionRepoError> for AppError {
    fn from(item: QuestionRepoError) -> Self {
        Self::QuestionRepo(item)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::QuestionRepo(QuestionRepoError::InvalidId) => {
                (StatusCode::NOT_FOUND, "Invalid Question Id provided")
            }
        };

        let body = json!({ "error": error_message });

        (status, Json(body)).into_response()
    }
}
