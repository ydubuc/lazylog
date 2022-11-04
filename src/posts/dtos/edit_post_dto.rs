use axum::http::StatusCode;
use serde::Deserialize;

use crate::{app::models::api_error::ApiError, auth::jwt::models::claims::Claims};

#[derive(Debug, Deserialize)]
pub struct EditPostDto {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl EditPostDto {
    pub fn to_sql(&self, claims: &Claims) -> Result<String, ApiError> {
        let mut sql = "UPDATE posts SET ".to_string();
        let mut clauses = Vec::new();

        let mut index: u8 = 1;

        // SET CLAUSES
        if self.title.is_some() {
            clauses.push(["title = $", &index.to_string()].concat());
            index += 1;
        }
        if self.content.is_some() {
            clauses.push(["content = $", &index.to_string()].concat());
            index += 1;
        }

        // CLAUSES BUILDER
        if clauses.len() == 0 {
            return Err(ApiError {
                code: StatusCode::BAD_REQUEST,
                message: "Received nothing to edit.".to_string(),
            });
        }

        for (i, clause) in clauses.iter().enumerate() {
            if i != 0 {
                sql.push_str(", ");
            }

            sql.push_str(&clause);
        }

        sql.push_str(&[" WHERE id = $", &index.to_string()].concat());
        sql.push_str(&[" AND user_id = '", &claims.id, "'"].concat());
        sql.push_str(" RETURNING *");

        println!("{}", sql);

        Ok(sql)
    }
}
