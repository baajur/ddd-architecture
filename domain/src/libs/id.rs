pub use identifier::{Display, FromStr, Identifier, ParseError};

pub mod uuid {
    use uuid::{Uuid, Version};

    pub fn generate() -> u128 {
        Uuid::new_v4().as_u128()
    }

    pub fn validate(value: u128) -> bool {
        if let Some(Version::Random) = Uuid::from_u128(value).get_version() {
            true
        } else {
            false
        }
    }
}
