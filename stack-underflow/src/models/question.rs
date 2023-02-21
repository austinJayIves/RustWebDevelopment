use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag(String);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuestionId(String);

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<Tag>>) -> Self {
        Self {
            id,
            title,
            content,
            tags: tags.unwrap_or_default(),
        }
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Question[id={}, title={}, content={:?}, tags={:?}]",
            self.id, self.title, self.content, self.tags
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
