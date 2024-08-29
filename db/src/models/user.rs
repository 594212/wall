use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Identifiable, Selectable, Queryable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub login: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}