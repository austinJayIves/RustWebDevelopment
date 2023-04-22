use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    QuestionRepo(QuestionRepoError),
    Pagination(PaginationError),
}

#[derive(Debug)]
pub enum QuestionRepoError {
    InvalidId,
    UnableToWrite,
    IdNotFound,
}

impl IntoResponse for QuestionRepoError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            QuestionRepoError::InvalidId => {
                (StatusCode::BAD_REQUEST, "Invalid Question Id provided")
            }
            QuestionRepoError::UnableToWrite => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unable to save question")
            }
            QuestionRepoError::IdNotFound => {
                (StatusCode::NOT_FOUND, "Unable to find provided question id")
            }
        };

        let body = json!({ "error": error_message });
        (status, Json(body)).into_response()
    }
}

#[derive(Debug)]
pub enum PaginationError {
    ValidationError,
    MaximumPageSizeExceeded(usize),
}

impl IntoResponse for PaginationError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, response) = match self {
            PaginationError::ValidationError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Unable to determine pagination".to_string(),
            ),
            PaginationError::MaximumPageSizeExceeded(size) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Maximum page size (100) exceeded: {}", size),
            ),
        };

        (status_code, Json(json!({ "error": response }))).into_response()
    }
}

impl From<QuestionRepoError> for AppError {
    fn from(item: QuestionRepoError) -> Self {
        Self::QuestionRepo(item)
    }
}

impl From<PaginationError> for AppError {
    fn from(item: PaginationError) -> Self {
        Self::Pagination(item)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::QuestionRepo(error) => error.into_response(),
            Self::Pagination(error) => error.into_response(),
        }
    }
}
