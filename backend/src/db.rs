use std::env;

use crate::models::media::ChunkBy;
use crate::models::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use dotenvy::dotenv;
use uuid::Uuid;

pub type Connection = AsyncPgConnection;
pub type PgPool = Pool<Connection>;

pub fn init_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mg = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    Pool::builder(mg)
        .build()
        .expect("Connection not build connection pool")
}

pub async fn create_serial(
    conn: &mut Connection,
    title: &str,
    description: &str,
) -> Result<Serial, Error> {
    use crate::schema::serials;

    let serial = NewSerial { title, description };

    let media = diesel::insert_into(serials::table)
        .values(&serial)
        .returning(Serial::as_returning())
        .get_result(conn)
        .await?;
    Ok(media)
}

pub async fn create_media(
    conn: &mut Connection,
    uuid: Uuid,
    model_id: i32,
    model_type: MediaType,
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

    let media = diesel::insert_into(medias::table)
        .values(&new_media)
        .returning(Media::as_returning())
        .get_result(conn)
        .await?;
    Ok(media)
}

pub async fn count_serials(conn: &mut Connection) -> Result<i64, Error> {
    use crate::schema::serials;
    serials::table.count().get_result(conn).await
}
pub async fn paging_serials(
    limit: i64,
    page: i64,
    conn: &mut Connection,
) -> Result<Vec<Serial>, Error> {
    use crate::schema::serials;
    let serials: Vec<Serial> = serials::table
        .select(Serial::as_returning())
        .limit(limit)
        .offset(page * limit)
        .load(conn)
        .await?;

    Ok(serials)
}

pub async fn retrieve_medias<T: Morph>(
    morphs: &Vec<T>,
    collection_type: CollectionType,
    conn: &mut Connection,
) -> Result<Vec<Vec<Media>>, Error> {
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
        .load(conn)
        .await?
        .chunk_by(morphs);

    Ok(medias)
}

pub async fn retrieve_categories(
    serials: &Vec<Serial>,
    limit: i64,
    offset: i64,
    conn: &mut Connection,
) -> Result<Vec<Vec<Category>>, Error> {
    use crate::schema::categories;

    let categories = CategorySerial::belonging_to(serials)
        .inner_join(categories::table)
        .limit(limit)
        .offset(offset)
        .select((CategorySerial::as_select(), Category::as_select()))
        .load(conn)
        .await?; // category_id category_id, ...

    Ok(categories
        .grouped_by(serials)
        .into_iter()
        .map(|c| c.into_iter().map(|(_, category)| category).collect())
        .collect())
}
pub async fn retrieve_category(
    limit: i64,
    offset: i64,
    serials: &Vec<Serial>,
    category_type: CategoryType,
    conn: &mut Connection,
) -> Result<Vec<impl Iterator<Item = Category>>, Error> {
    use crate::schema::categories;

    let categories = CategorySerial::belonging_to(serials)
        .inner_join(categories::table)
        .filter(categories::category_type.eq(category_type))
        .limit(limit)
        .offset(offset)
        .select((CategorySerial::as_select(), Category::as_select()))
        .load(conn)
        .await?; // category_id category_id, ...

    Ok(categories
        .grouped_by(serials)
        .into_iter()
        .map(|c| c.into_iter().map(|(_, category)| category))
        .collect())
}
