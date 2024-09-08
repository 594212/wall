use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow};
use std::io::Write;

use super::{MediaType, Morph};

#[derive(Identifiable, Selectable, Queryable, Debug)]
#[diesel(table_name=crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub text: Option<String>,
    pub model_id: i32,
    pub model_type: CommentType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = crate::schema::sql_types::CommentType)]
pub enum CommentType {
    Comment,
    Episode,
    Serial,
}

impl ToSql<crate::schema::sql_types::CommentType, Pg> for CommentType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            CommentType::Serial => out.write_all(b"serial")?,
            CommentType::Comment => out.write_all(b"child")?,
            CommentType::Episode => out.write_all(b"episode")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::CommentType, Pg> for CommentType {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"serial" => Ok(CommentType::Serial),
            b"episode" => Ok(CommentType::Episode),
            b"child" => Ok(CommentType::Comment),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Morph for Comment {
    fn model_id(&self) -> i32 {
        self.id
    }

    fn media_type() -> MediaType {
        MediaType::Comment
    }

    fn coomment_type() -> CommentType {
        CommentType::Comment
    }
}
