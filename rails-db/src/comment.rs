use std::io::Write;
use chrono::NaiveDateTime;
use diesel::{AsExpression, Associations, FromSqlRow, Identifiable, Insertable, Queryable, Selectable, SqlType};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use crate::schema::sql_types::CommentType;

#[derive(Identifiable, Selectable, Queryable, Debug)]
#[diesel(table_name=comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
    Comment,
    Episode,
    Serial,
}

impl ToSql<CommentType, Pg> for CommentTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            CommentTypeEnum::Serial => out.write_all(b"serial")?,
            CommentTypeEnum::Comment => out.write_all(b"child")?,
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
            b"child" => Ok(CommentTypeEnum::Comment),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}