// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "category_type"))]
    pub struct CategoryType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "comment_type"))]
    pub struct CommentType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "model_type"))]
    pub struct ModelType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CategoryType;

    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        category_type -> CategoryType,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories_serials (category_id, serial_id) {
        category_id -> Int4,
        serial_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CommentType;

    comments (id) {
        id -> Int4,
        text -> Nullable<Text>,
        model_id -> Int4,
        model_type -> CommentType,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    episodes (id) {
        id -> Int4,
        number -> Int4,
        name -> Nullable<Varchar>,
        serial_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    likes (user_id, episode_id) {
        user_id -> Int4,
        episode_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModelType;

    medias (id) {
        id -> Int4,
        uuid -> Uuid,
        model_id -> Int8,
        model_type -> ModelType,
        file_name -> Varchar,
        mime_type -> Varchar,
        conversion -> Text,
        size -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ratings (user_id, serial_id) {
        number -> Int2,
        user_id -> Int4,
        serial_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    serials (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        serial_count -> Int4,
        rating -> Float4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    views (user_id, episode_id) {
        user_id -> Int4,
        episode_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(categories_serials -> categories (category_id));
diesel::joinable!(categories_serials -> serials (serial_id));
diesel::joinable!(episodes -> serials (serial_id));
diesel::joinable!(likes -> episodes (episode_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(ratings -> serials (serial_id));
diesel::joinable!(ratings -> users (user_id));
diesel::joinable!(views -> episodes (episode_id));
diesel::joinable!(views -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    categories_serials,
    comments,
    episodes,
    likes,
    medias,
    ratings,
    serials,
    users,
    views,
);
