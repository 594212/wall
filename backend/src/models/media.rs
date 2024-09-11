use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Write;
use uuid::Uuid;

use super::Morph;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::medias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    #[serde(skip)]
    pub uuid: Uuid,
    pub model_id: i32,
    pub model_type: MediaType,
    pub collection_type: CollectionType,
    pub file_name: String,
    pub mime_type: String,
    pub conversion: String,
    pub size: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Media {
    pub fn get_default() -> String {
        String::from("assert/default/avatar.png")
    }
    pub fn to_path(&self) -> String {
        format!("assets/{}/{}", self.uuid, self.file_name)
    }
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::medias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia<'a> {
    pub uuid: Uuid,
    pub model_id: i32,
    pub model_type: MediaType,
    pub collection_type: CollectionType,
    pub file_name: &'a str,
    pub mime_type: &'a str,
    pub conversion: &'a str,
    pub size: i64,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Deserialize, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::ModelType)]
pub enum MediaType {
    Serial,
    Episode,
    Comment,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Deserialize, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::CollectionType)]
pub enum CollectionType {
    Video,
    Avatar,
}

impl FromSql<crate::schema::sql_types::CollectionType, Pg> for CollectionType {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"avatar" => Ok(CollectionType::Avatar),
            b"video" => Ok(CollectionType::Video),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::CollectionType, Pg> for CollectionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            CollectionType::Video => out.write_all(b"video")?,
            CollectionType::Avatar => out.write_all(b"avatar")?,
        };
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ModelType, Pg> for MediaType {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"serial" => Ok(MediaType::Serial),
            b"episode" => Ok(MediaType::Episode),
            b"comment" => Ok(MediaType::Comment),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::ModelType, Pg> for MediaType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            MediaType::Serial => out.write_all(b"serial")?,
            MediaType::Episode => out.write_all(b"episode")?,
            MediaType::Comment => out.write_all(b"episode")?,
        };
        Ok(IsNull::No)
    }
}

pub trait ChunkBy<'a, Parent>: IntoIterator + Sized {
    fn chunk_by(self, parents: &'a [Parent]) -> Vec<Vec<Self::Item>>;
}

impl<'a, Owner: 'a, Iter> ChunkBy<'a, Owner> for Iter
where
    Iter: IntoIterator<Item = Media>,
    Owner: Morph,
{
    fn chunk_by(self, parents: &'a [Owner]) -> Vec<Vec<Self::Item>> {
        use std::collections::HashMap;

        let id_indices: HashMap<_, _> = parents
            .iter()
            .enumerate()
            .map(|(i, u)| (u.model_id(), i))
            .collect();
        let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        for media in self {
            let index = id_indices[&media.model_id];
            result[index].push(media);
        }
        result
    }
}
