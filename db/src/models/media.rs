use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow};
use std::io::Write;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::medias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub uuid: Uuid,
    pub model_id: i32,
    pub model_type: ModelType,
    pub collection_type: CollectionType,
    pub file_name: String,
    pub mime_type: String,
    pub conversion: String,
    pub size: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::medias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia<'a> {
    pub uuid: Uuid,
    pub model_id: i32,
    pub model_type: ModelType,
    pub collection_type: CollectionType,
    pub file_name: &'a str,
    pub mime_type: &'a str,
    pub conversion: &'a str,
    pub size: i64,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = crate::schema::sql_types::ModelType)]
pub enum ModelType {
    Serial,
    Episode,
    Comment,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
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

impl FromSql<crate::schema::sql_types::ModelType, Pg> for ModelType {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"serial" => Ok(ModelType::Serial),
            b"episode" => Ok(ModelType::Episode),
            b"comment" => Ok(ModelType::Comment),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::ModelType, Pg> for ModelType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            ModelType::Serial => out.write_all(b"serial")?,
            ModelType::Episode => out.write_all(b"episode")?,
            ModelType::Comment => out.write_all(b"episode")?,
        };
        Ok(IsNull::No)
    }
}

pub trait HasMedia {
    fn model_id(&self) -> i32;
}
pub trait ChunkBy<'a, Parent>: IntoIterator {
    fn media_chunk_by(self, parents: &'a [Parent]) -> Vec<Vec<Self::Item>>;
}

impl<'a, Owner: 'a> ChunkBy<'a, Owner> for Vec<Media>
where
    &'a Owner: HasMedia,
{
    fn media_chunk_by(self, parents: &'a [Owner]) -> Vec<Vec<Self::Item>> {
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

pub fn media2_chunk_by<'a, T: HasMedia>(childs: Vec<Media>, parents: &'a [T]) -> Vec<Vec<Media>> {
    use std::collections::HashMap;

    let id_indices: HashMap<_, _> = parents
        .iter()
        .enumerate()
        .map(|(i, u)| (u.model_id(), i))
        .collect();
    let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
    for media in childs {
        let index = id_indices[&media.model_id];
        result[index].push(media);
    }
    result
}
