use super::question::QuestionId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct Answer {
    pub id: AnswerId,
    pub question_id: QuestionId,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct AnswerId(String);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct AnswerBody {
    pub content: String,
}

impl Answer {
    pub fn new(question_id: QuestionId, content: String) -> Self {
        Self {
            id: AnswerId(Uuid::new_v4().to_string()),
            question_id,
            content,
        }
    }

    pub fn id(&self) -> &AnswerId {
        &self.id
    }
}
