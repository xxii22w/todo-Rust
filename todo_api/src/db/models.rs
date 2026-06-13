use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TodoModel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoPartial {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}
