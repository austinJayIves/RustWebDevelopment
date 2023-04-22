use crate::models::{
    answer::Answer,
    error::{AppError, QuestionRepoError},
    question::QuestionId,
    store::Store,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::str::FromStr;
use std::sync::Arc;

pub async fn post_answer(
    Extension(store): Extension<Arc<Store>>,
    Path(question_id): Path<String>,
    Json(content): Json<String>,
) -> Result<impl IntoResponse, AppError> {
    let question_id =
        QuestionId::from_str(&question_id).map_err(|_| QuestionRepoError::InvalidId)?;
    let answer = Answer::new(question_id.clone(), content);

    let question_store = store.questions.read().await;
    if !question_store.contains_key(&question_id) {
        return Err(QuestionRepoError::IdNotFound.into());
    }

    let mut answer_store = store.answers.write().await;
    answer_store.insert(answer.id().clone(), answer.clone());

    Ok((StatusCode::OK, Json(answer)))
}
