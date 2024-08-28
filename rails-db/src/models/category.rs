use crate::models::episodes::Serial;
use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::prelude::*;
use std::io::Write;
use diesel::{AsExpression, FromSqlRow};

#[derive(Identifiable, Selectable, Queryable, Debug)]
#[diesel(table_name=crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_type: CategoryType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Category))]
#[diesel(belongs_to(Serial))]
#[diesel(table_name = crate::schema::categories_serials)]
#[diesel(primary_key(category_id, serial_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CategorySerial {
    pub category_id: i32,
    pub serial_id: i32,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type=crate::schema::sql_types::CategoryType)]
pub enum CategoryType {
    Tag,
    Genre,
    Author,
    Year,
    Status,
}
impl ToSql<crate::schema::sql_types::CategoryType, Pg> for CategoryType {
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

impl FromSql<crate::schema::sql_types::CategoryType, Pg> for CategoryType {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
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

