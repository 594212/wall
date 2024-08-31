pub mod models;
mod repository;
pub mod schema;

use std::env;

use crate::models::episodes::*;
use crate::models::media::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use models::category::*;
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
) -> Result<Serial, Error> {
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
    model_id: i32,
    model_type: ModelType,
    collection_type: CollectionType,
    file_name: &str,
    mime_type: &str,
    conversion: &str,
    size: i64,
) -> Result<Media, Error> {
    use crate::schema::medias;
    let new_media = NewMedia {
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
) -> Result<Vec<Serial>, Error> {
    use crate::schema::serials;
    serials::table
        .select(Serial::as_returning())
        .limit(page_size)
        .offset(offset)
        .load(conn)
}

pub fn retrieve_medias(
    serials: Vec<Serial>,
    model_type: ModelType,
    collection_type: CollectionType,
    conn: &mut PgConnection,
) -> Result<Vec<(Serial, Vec<Media>)>, Error> {
    use crate::schema::medias;
    let model_ids: Vec<i32> = serials.iter().map(|s| s.model_id()).collect();
    let medias: Vec<Media> = medias::table
        .select(Media::as_select())
        .filter(
            medias::model_id.eq_any(model_ids).and(
                medias::model_type
                    .eq(model_type)
                    .and(medias::collection_type.eq(collection_type)),
            ),
        )
        .load(conn)?;
    let med = media2_chunk_by(medias, &serials);

    Ok(serials.into_iter().zip(med).collect())
}

pub fn retrieve_categories(
    serials: Vec<Serial>,
    category_type: CategoryType,
    conn: &mut PgConnection,
) -> Result<Vec<(Serial, Vec<Category>)>, Error> {
    use crate::schema::categories;

    let categories = CategorySerial::belonging_to(&serials)
        .inner_join(categories::table)
        .filter(categories::category_type.eq(category_type))
        .select((CategorySerial::as_select(), Category::as_select()))
        .load(conn)?;

    Ok(categories
        .grouped_by(&serials)
        .into_iter()
        .zip(serials)
        .map(|(c, serial)| {
            (
                serial,
                c.into_iter().map(|(_, category)| category).collect(),
            )
        })
        .collect())
}
