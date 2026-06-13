use sqlx::MySqlPool;
use thiserror::Error;

use crate::db::models::UpdateTodo;
use crate::db::models::UpdateTodoPartial;
use crate::{
    db::{DbConnectionPoolError, models::TodoModel},
    handlers::todo::models::CreateTodoRequest,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool init error: {0}")]
    ConnectionPool(#[from] DbConnectionPoolError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub struct Service {
    conn_pool: MySqlPool,
}

impl Service {
    pub fn new(conn_pool: MySqlPool) -> Result<Self, Error> {
        Ok(Self { conn_pool })
    }

    pub async fn create(
        &self,
        user_id: i32,
        request: CreateTodoRequest,
    ) -> Result<TodoModel, Error> {
        let mut tx = self.conn_pool.begin().await?;

        let insert_sql = "INSERT INTO todos (user_id, title, description) VALUES (?,?,?)";
        let result = sqlx::query(insert_sql)
            .bind(user_id)
            .bind(&request.title)
            .bind(&request.description)
            .execute(&mut *tx)
            .await?;

        let new_id = result.last_insert_id();

        let select_sql =
            "SELECT id, user_id, title, description, created, updated FROM todos WHERE id = ?";
        let create_todo = sqlx::query_as(select_sql)
            .bind(new_id)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(create_todo)
    }

    pub async fn list(&self, user_id: i32) -> Result<Vec<TodoModel>, Error> {
        let select_sql =
            "SELECT id, user_id, title, description, created, updated FROM todos WHERE user_id = ?";
        let result_lists = sqlx::query_as::<_, TodoModel>(select_sql)
            .bind(user_id)
            .fetch_all(&self.conn_pool)
            .await?;
        Ok(result_lists)
    }

    pub async fn get(&self, user_id: i32, id: i32) -> Result<TodoModel, Error> {
        let select_sql = "SELECT id, user_id, title, description, created, updated FROM todos WHERE user_id = ? AND id =?";
        let result = sqlx::query_as::<_, TodoModel>(select_sql)
            .bind(user_id)
            .bind(id)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(result)
    }

    pub async fn delete(&self, user_id: i32, id: i32) -> Result<(), Error> {
        let delete_sql = "DELETE FROM todos WHERE id = ? AND user_id = ?";

        let result = sqlx::query(delete_sql)
            .bind(id)
            .bind(user_id)
            .execute(&self.conn_pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        Ok(())
    }

    pub async fn partial_update(
        &self,
        user_id: i32,
        id: i32,
        request: UpdateTodoPartial,
    ) -> Result<(), Error> {
        let check_sql = "SELECT 1 FROM todos WHERE id = ? AND user_id = ?";
        let exists = sqlx::query(check_sql)
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.conn_pool) // 返回 Option<MySqlRow>
            .await?;

        if exists.is_none() {
            return Err(sqlx::Error::RowNotFound.into());
        }
        let update_sql = "UPDATE todos SET title = COALESCE(?,title), description = COALESCE(?,description) WHERE id = ? AND user_id = ?";

        sqlx::query(update_sql)
            .bind(request.title)
            .bind(request.description)
            .bind(id)
            .bind(user_id)
            .execute(&self.conn_pool)
            .await?;

        Ok(())
    }

    pub async fn update(&self, user_id: i32, id: i32, request: UpdateTodo) -> Result<(), Error> {
        let update_sql = "UPDATE todos SET title = ?, description = ? WHERE id = ? AND user_id = ?";
        sqlx::query(update_sql)
            .bind(request.title)
            .bind(request.description)
            .bind(id)
            .bind(user_id)
            .execute(&self.conn_pool)
            .await?;

        Ok(())
    }
}
