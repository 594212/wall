use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::schema::users;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    login: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}