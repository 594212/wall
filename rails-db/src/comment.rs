use std::io::Write;
use chrono::NaiveDateTime;
use diesel::{AsExpression, FromSqlRow, Identifiable, Insertable, SqlType};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use crate::schema::medias::model_id;

pub struct Comment {
    id: i32,
    text: String,
    model_id: i32,
    model_type: CommentTypeEnum,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
pub enum CommentTypeEnum {
    Child,
    Episode,
    Serial,
}

#[derive(SqlType)]
#[diesel(postgres_type(name = COMMENT_TYPE))]
pub struct CommentType;

impl ToSql<CommentType, Pg> for CommentTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            CommentTypeEnum::Serial => out.write_all(b"serial")?,
            CommentTypeEnum::Child => out.write_all(b"child")?,
            CommentTypeEnum::Episode => out.write_all(b"episode")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<CommentType, Pg> for CommentTypeEnum {
    fn from_sql(bytes: Pg::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"serial" => Ok(CommentTypeEnum::Serial),
            b"episode" => Ok(CommentTypeEnum::Episode),
            b"child" => Ok(CommentTypeEnum::Child),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}