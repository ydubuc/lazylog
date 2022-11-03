use axum::http::StatusCode;
use serde::Deserialize;

use crate::{app::models::api_error::ApiError, users::models::user::USER_SORTABLE_FIELDS};

#[derive(Deserialize, Debug)]
pub struct GetUsersFilterDto {
    pub id: Option<String>,
    pub username: Option<String>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u8>,
}

impl GetUsersFilterDto {
    pub fn to_sql(&self) -> Result<String, ApiError> {
        let mut sql = "SELECT * FROM users".to_string();
        let mut clauses = Vec::new();

        let mut sort_field = "created_at".to_string();
        let mut sort_order = "DESC".to_string();
        let mut page_limit: u8 = 50;

        if let Some(id) = &self.id {
            clauses.push(["id = ", id].concat());
        }
        if let Some(username) = &self.username {
            clauses.push(["username_key LIKE '%", &username.to_lowercase(), "%'"].concat())
        }
        if let Some(sort) = &self.sort {
            let sort_params: Vec<&str> = sort.split(",").collect();

            if sort_params.len() != 2 {
                return Err(ApiError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Malformed sort query.".to_string(),
                });
            }
            if !USER_SORTABLE_FIELDS.contains(&sort_params[0]) {
                return Err(ApiError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid sort field.".to_string(),
                });
            }

            sort_field = sort_params[0].to_string();
            sort_order = sort_params[1].to_uppercase();

            let direction = match sort_order.as_str() {
                "ASC" => ">",
                "DESC" => "<",
                _ => {
                    return Err(ApiError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Malformed sort query.".to_string(),
                    })
                }
            };

            if let Some(cursor) = &self.cursor {
                clauses.push([&sort_field, " ", direction, " ", cursor].concat());
            }
        }

        let mut has_inserted_where = false;

        for clause in clauses {
            if !has_inserted_where {
                sql.push_str(" WHERE ");
                has_inserted_where = true;
            } else {
                sql.push_str(" AND ");
            }

            sql.push_str(&clause);
        }

        sql.push_str(&[" ORDER BY ", &sort_field, " ", &sort_order].concat());

        if let Some(limit) = self.limit {
            page_limit = limit;
        }

        sql.push_str(&[" LIMIT ", &page_limit.to_string()].concat());

        println!("{:?}", sql);

        return Ok(sql.to_string());
    }
}