mod error;
pub mod models;
mod repository;
pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::QueryDsl;
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
pub use error::DbError;
use models::media::ChunkBy;
pub use models::*;
use uuid::Uuid;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type Conn = PgConnection;

pub fn init_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Connection not build connection pool")
}
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
) -> Result<Serial, DbError> {
    use crate::schema::serials;

    let serial = NewSerial { title, description };

    let media = diesel::insert_into(serials::table)
        .values(&serial)
        .returning(Serial::as_returning())
        .get_result(conn)?;
    Ok(media)
}

pub fn create_media(
    conn: &mut PgConnection,
    uuid: Uuid,
    model_id: i32,
    model_type: MediaType,
    collection_type: CollectionType,
    file_name: &str,
    mime_type: &str,
    conversion: &str,
    size: i64,
) -> Result<Media, DbError> {
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

    let media = diesel::insert_into(medias::table)
        .values(&new_media)
        .returning(Media::as_returning())
        .get_result(conn)?;
    Ok(media)
}

pub fn paging_serials(
    limit: i64,
    page: i64,
    conn: &mut PgConnection,
) -> Result<Vec<Serial>, DbError> {
    use crate::schema::serials;
    let serials: Vec<Serial> = serials::table
        .select(Serial::as_returning())
        .limit(limit)
        .offset(page * limit)
        .load(conn)?;

    Ok(serials)
}

pub fn retrieve_medias<T: Morph>(
    morphs: Vec<T>,
    collection_type: CollectionType,
    conn: &mut PgConnection,
) -> Result<Vec<Vec<Media>>, DbError> {
    use crate::schema::medias;
    let model_ids: Vec<i32> = morphs.iter().map(|s| s.model_id()).collect();
    let medias = medias::table
        .select(Media::as_select())
        .filter(
            medias::model_id.eq_any(model_ids).and(
                medias::model_type
                    .eq(T::media_type())
                    .and(medias::collection_type.eq(collection_type)),
            ),
        )
        .load(conn)?
        .chunk_by(&morphs);

    Ok(medias)
}

pub fn retrieve_categories(
    serials: Vec<Serial>,
    category_type: CategoryType,
    conn: &mut PgConnection,
) -> Result<Vec<(Serial, Vec<Category>)>, DbError> {
    use crate::schema::categories;

    let categories = CategorySerial::belonging_to(&serials)
        .inner_join(categories::table)
        .filter(categories::category_type.eq(category_type))
        .select((CategorySerial::as_select(), Category::as_select()))
        .load(conn)?; // category_id category_id, ...

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
