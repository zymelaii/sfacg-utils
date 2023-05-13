use anyhow::Result;

pub type Timestamp = i64;
pub type Id = i32;
pub enum Type {
    Comic,
    Novel,
    Album,
}
pub enum VerifyType {
    None(String),
}

impl Type {
    #[inline]
    pub fn from(value: i32) -> Self {
        match value {
            1 => Type::Comic,
            2 => Type::Novel,
            3 => Type::Album,
            _ => unreachable!(),
        }
    }
}

impl VerifyType {
    #[inline]
    pub fn from(r#type: i32, info: String) -> Self {
        match r#type {
            0 => VerifyType::None(info),
            _ => unreachable!(),
        }
    }
}

#[inline]
pub fn to_timestamp(date: &str) -> Result<Timestamp> {
    Ok(date.parse::<dateparser::DateTimeUtc>()?.0.timestamp())
}

mod album;
mod author;
mod comic;
mod favorites;
mod novel;
mod private;
mod user;

pub use album::*;
pub use author::*;
pub use comic::*;
pub use favorites::*;
pub use novel::*;
pub use private::*;
pub use user::*;
