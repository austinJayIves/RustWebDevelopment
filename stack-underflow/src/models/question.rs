pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Vec<Tag>,
}

pub struct Tag(String);
pub struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<Tag>>) -> Self {
        Self {
            id,
            title,
            content,
            tags: tags.unwrap_or(vec![]),
        }
    }
}

impl From<String> for Tag {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<String> for QuestionId {
    fn from(item: String) -> Self {
        Self(item)
    }
}
