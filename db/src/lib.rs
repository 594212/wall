pub mod models;
mod repository;
pub mod schema;

use std::env;

use crate::models::episodes;
use crate::models::media;
use diesel::QueryDsl;
use diesel::{pg::PgConnection, Connection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::media::Media;
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_serial(
    conn: &mut PgConnection,
    title: &str,
    description: &str,
) -> Result<crate::episodes::Serial, diesel::result::Error> {
    use crate::episodes::{NewSerial, Serial};
    use crate::schema::serials;

    let serial = NewSerial { title, description };

    diesel::insert_into(serials::table)
        .values(&serial)
        .returning(Serial::as_returning())
        .get_result(conn)
}

pub fn create_media(
    conn: &mut PgConnection,
    uuid: Uuid,
    model_id: i64,
    model_type: crate::media::ModelType,
    collection_type: crate::media::CollectionType,
    file_name: &str,
    mime_type: &str,
    conversion: &str,
    size: i64,
) -> Result<crate::media::Media, diesel::result::Error> {
    use crate::schema::medias;
    let new_media = crate::media::NewMedia {
        uuid,
        model_id,
        model_type,
        collection_type,
        file_name,
        mime_type,
        conversion,
        size,
    };

    diesel::insert_into(medias::table)
        .values(&new_media)
        .returning(Media::as_returning())
        .get_result(conn)
}

pub fn paging_serials(
    page_size: i64,
    offset: i64,
    conn: &mut PgConnection,
) -> Result<Vec<crate::episodes::Serial>, diesel::result::Error> {
    use crate::episodes::Serial;
    use crate::schema::serials;
    serials::table
        .select(Serial::as_returning())
        .limit(page_size)
        .offset(offset)
        .load(conn)
}

pub fn media() {}
