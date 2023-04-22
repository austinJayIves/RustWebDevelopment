use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuestionBody {
    title: String,
    content: String,
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Question {
    id: QuestionId,
    #[serde(flatten)]
    body: QuestionBody,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag(String);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuestionId(String);

impl Question {
    pub fn new(id: QuestionId, body: QuestionBody) -> Self {
        Self { id, body }
    }

    pub fn id(&self) -> QuestionId {
        self.id.clone()
    }
}

impl QuestionBody {
    pub fn into_question(self, question_id: QuestionId) -> Question {
        Question {
            id: question_id,
            body: self,
        }
    }

    pub fn new(title: String, content: String, tags: Option<Vec<Tag>>) -> Self {
        Self {
            title,
            content,
            tags: tags.unwrap_or_default(),
        }
    }
}

impl From<QuestionBody> for Question {
    fn from(body: QuestionBody) -> Self {
        let question_id = QuestionId(Uuid::new_v4().to_string());
        body.into_question(question_id)
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Question[id={}, title={}, content={:?}, tags={:?}]",
            self.id, self.body.title, self.body.content, self.body.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "QuestionId[{}]", self.0)
    }
}

impl FromStr for Tag {
    type Err = std::io::Error;

    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        match tag.is_empty() {
            false => Ok(Tag(tag.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No tag provided")),
        }
    }
}

impl From<String> for Tag {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}
