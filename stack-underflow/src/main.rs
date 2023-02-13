use stack_underflow::models::question::{Question, QuestionId, Tag};
use std::str::FromStr;

fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec![Tag::from_str("faq").expect("No tag provided")]),
    );
    println!("{}", question);
}
