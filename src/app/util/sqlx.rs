use std::borrow::Cow;

use sqlx::error::DatabaseError;

#[non_exhaustive]
pub struct SqlStateCodes;

impl SqlStateCodes {
    pub const UNIQUE_VIOLATION: &str = "23505";
}

pub fn get_code_from_db_err(db_err: &dyn DatabaseError) -> Option<String> {
    match db_err.code() {
        Some(code) => match code {
            Cow::Borrowed(val) => return Some(val.to_owned()),
            Cow::Owned(val) => return Some(val),
        },
        None => return None,
    }
}
