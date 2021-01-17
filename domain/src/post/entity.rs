use crate::libs::id::{uuid, Display, FromStr, Identifier};

#[derive(Identifier, Eq, PartialEq, Display, FromStr, Debug)]
#[identifier(with = "uuid")]
pub struct Id(u128);

#[derive(Debug, Eq, PartialEq)]
pub struct Post {
    id: Id,
    content: String,
}

impl Post {
    pub fn new(content: &str) -> Post {
        Post {
            id: Id::generate(),
            content: content.to_owned(),
        }
    }

    pub fn build_from_repository(id: Id, content: String) -> Post {
        Post { id, content }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
