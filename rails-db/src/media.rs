use std::io::Write;
use diesel::{AsExpression, FromSqlRow, Identifiable, Queryable, Selectable, SqlType};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = medias)]
pub struct Media {
    pub id: i32,
    uuid: Uuid,
    model_id: i64,
    model_type: MediaTypeEnum,
    file_name: String,
    mime_type: String,
    conversion: String,
    size: i32,
}
#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type=(name = MediaType))]
pub enum MediaTypeEnum {
    Serial,
    Episode,
    Comment,
}

#[derive(SqlType)]
#[diesel(postgres_type(name = "media_type"))]
pub struct MediaType;

impl FromSql<MediaType, Pg> for MediaTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"serial" => Ok(MediaTypeEnum::Serial),
            b"episode" => Ok(MediaTypeEnum::Episode),
            b"comment" => Ok(MediaTypeEnum::Comment),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}

impl ToSql<MediaType, Pg> for MediaTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            MediaTypeEnum::Serial => out.write_all(b"serial")?,
            MediaTypeEnum::Episode => out.write_all(b"episode")?,
            MediaTypeEnum::Comment => out.write_all(b"episode")?,
        };
        Ok(IsNull::No)
    }
}

