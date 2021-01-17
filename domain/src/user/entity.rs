use crate::libs::id::{uuid, Display, FromStr, Identifier};

#[derive(Identifier, Eq, PartialEq, Display, FromStr, Debug)]
#[identifier(with = "uuid")]
pub struct Id(u128);

#[derive(Debug, Eq, PartialEq)]
pub struct User {
    id: Id,
    nickname: String,
}

impl User {
    pub fn new(nickname: &str) -> User {
        User {
            id: Id::generate(),
            nickname: nickname.to_owned(),
        }
    }

    pub fn build_from_repository(id: Id, nickname: String) -> User {
        User { id, nickname }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }
}
