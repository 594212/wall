use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::serials::description;

#[derive(Queryable, Serialize, Identifiable, Debug, PartialEq)]
#[diesel(table_name = books)]
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
#[diesel(table_name = books)]
pub struct NewSerial<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belong_to(Serial))]
#[diesel(table_name = serials)]
pub struct Episode {
    pub id: i32,
    pub name: Option<String>,
    pub number: i32,
    pub serial_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    login: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Episode))]
#[diesel(table_name = views)]
#[diesel(primary_key(user_id, episode_id))]
pub struct View {
    pub user_id: i32,
    pub episode_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Episode))]
#[diesel(table_name = likes)]
#[diesel(primary_key(user_id, episode_id))]
pub struct Like {
    pub user_id: i32,
    pub episode_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Serial))]
#[diesel(table_name = ratings)]
#[diesel(primary_key(user_id, serial_id))]
pub struct Rating {
    pub number: i32,
    pub user_id: i32,
    pub serial_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
