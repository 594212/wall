use std::io::Write;
use chrono::NaiveDateTime;
use diesel::{AsExpression, Associations, FromSqlRow, Identifiable, Queryable, Selectable, SqlType};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use crate::media::MediaTypeEnum;
use crate::schema::episodes::name;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub c_type: CategoryType,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Category))]
#[disel(belongs_to(Serial))]
#[diesel(table_name = categories_serials)]
#[diesel(primary_key(category_id, serial_id))]
pub struct CategorySerial {
    pub category_id: i32,
    pub serial_id: i32,
}

#[derive(SqlType)]
#[diesel(postgres_type = "c_type")]
pub struct CType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type=CType)]
pub enum CategoryType {
    Tag,
    Genre,
    Author,
    Year,
    Status,
}
impl ToSql<CType, Pg> for CategoryType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            CategoryType::Tag => out.write_all(b"tag")?,
            CategoryType::Genre => out.write_all(b"genre")?,
            CategoryType::Author => out.write_all(b"author")?,
            CategoryType::Year => out.write_all(b"year")?,
            CategoryType::Status => out.write_all(b"status")?
        }
        Ok(IsNull::No)
    }
}

impl FromSql<CType, Pg> for CategoryType {
    fn from_sql(bytes: Pg::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"tag" => Ok(CategoryType::Tag),
            b"genre" => Ok(CategoryType::Genre),
            b"author" => Ok(CategoryType::Author),
            b"year" => Ok(CategoryType::Year),
            b"status" => Ok(CategoryType::Status),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}

