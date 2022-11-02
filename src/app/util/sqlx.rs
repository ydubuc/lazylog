use std::borrow::Cow;

use sqlx::error::DatabaseError;

pub fn get_code_from_db_err(db_err: &dyn DatabaseError) -> Option<String> {
    let code = db_err.code();
    if code.is_none() {
        return None;
    } else {
        match code.unwrap() {
            Cow::Borrowed(val) => return Some(val.to_owned()),
            Cow::Owned(val) => return Some(val),
        }
    }
}
