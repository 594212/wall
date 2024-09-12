use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use super::{CommentType, MediaType, Morph};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize)]
#[diesel(table_name = crate::schema::serials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Serial {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub serial_count: i32,
    pub rating: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::serials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSerial<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Serial))]
#[diesel(table_name = crate::schema::episodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Episode {
    pub id: i32,
    pub name: Option<String>,
    pub number: i32,
    pub serial_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Morph for Serial {
    fn model_id(&self) -> i32 {
        self.id
    }

    fn media_type() -> MediaType {
        MediaType::Serial
    }

    fn coomment_type() -> CommentType {
        CommentType::Serial
    }
}
impl Morph for Episode {
    fn model_id(&self) -> i32 {
        self.id
    }

    fn media_type() -> MediaType {
        MediaType::Episode
    }

    fn coomment_type() -> CommentType {
        CommentType::Episode
    }
}
