// Model ou Entity?

#[derive(Debug, Eq, PartialEq)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> User {
        User {
            name: name.to_owned(),
        }
    }
}
