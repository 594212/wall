use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::serials;
use crate::schema::episodes;

#[derive(Queryable, Serialize, Identifiable, Debug, PartialEq)]
#[diesel(table_name = serials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Serial {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub serial_count: i32,
    pub rating: f32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = serials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSerial<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belong_to(Serial))]
#[diesel(table_name = episodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Episode {
    pub id: i32,
    pub name: Option<String>,
    pub number: i32,
    pub serial_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}