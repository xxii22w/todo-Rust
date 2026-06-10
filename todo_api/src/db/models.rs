use diesel::{
    Selectable,
    associations::{Associations, Identifiable},
    data_types::MysqlTime,
    prelude::{AsChangeset, Insertable, Queryable},
};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created: MysqlTime,
    pub updated: MysqlTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::db::schema::todos)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TodoModel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created: MysqlTime,
    pub updated: MysqlTime,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct CreateTodo {
    pub user_id: i32,
    pub title: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct UpdateTodoPartial {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}
