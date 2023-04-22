use crate::models::{
    error::{AppError, QuestionRepoError},
    pagination::{PaginatedResponse, Pagination},
    question::{Question, QuestionBody, QuestionId},
    store::Store,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, instrument};

#[instrument(skip(store))]
pub async fn get_questions(
    Extension(store): Extension<Arc<Store>>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let question_store = store.questions.read().await;

    let pagination = pagination.validate()?;
    info!(
        "Returning Page[offset={}, max_results={}]",
        pagination.offset, pagination.max_results
    );

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(
            pagination,
            question_store
                .values()
                .skip(pagination.offset)
                .take(pagination.max_results)
                .cloned()
                .collect::<Vec<_>>(),
        )),
    ))
}

#[instrument(skip(store))]
pub async fn get_question(
    Path(id): Path<String>,
    Extension(store): Extension<Arc<Store>>,
) -> Result<impl IntoResponse, AppError> {
    let question_store = store.questions.read().await;

    question_store
        .get(&QuestionId::from_str(&id).map_err(|_| QuestionRepoError::InvalidId)?)
        .ok_or_else(|| QuestionRepoError::IdNotFound.into())
        .map(|question| (StatusCode::OK, Json(question.clone())))
}

#[instrument(skip(store))]
pub async fn post_question(
    Extension(store): Extension<Arc<Store>>,
    Json(body): Json<QuestionBody>,
) -> Result<impl IntoResponse, AppError> {
    let question = Question::from(body);
    let mut question_store = store.questions.write().await;
    question_store.insert(question.id(), question.clone());

    Ok((StatusCode::OK, Json(question)))
}

#[instrument(skip(store))]
pub async fn put_question(
    Extension(store): Extension<Arc<Store>>,
    Path(question_id): Path<String>,
    Json(body): Json<QuestionBody>,
) -> Result<impl IntoResponse, AppError> {
    let question_id =
        QuestionId::from_str(&question_id).map_err(|_| QuestionRepoError::InvalidId)?;
    let question = Question::new(question_id, body);

    let mut question_store = store.questions.write().await;
    question_store.insert(question.id(), question.clone());

    Ok((StatusCode::OK, Json(question)))
}

#[instrument(skip(store))]
pub async fn delete_question(
    Extension(store): Extension<Arc<Store>>,
    Path(question_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let question_id =
        QuestionId::from_str(&question_id).map_err(|_| QuestionRepoError::InvalidId)?;

    let mut question_store = store.questions.write().await;
    question_store
        .remove(&question_id)
        .ok_or_else(|| QuestionRepoError::IdNotFound.into())
        .map(|_| StatusCode::OK)
}
