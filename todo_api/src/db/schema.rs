// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        created -> Datetime,
        updated -> Datetime,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        password -> Text,
        created -> Datetime,
        updated -> Datetime,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(todos, users,);
