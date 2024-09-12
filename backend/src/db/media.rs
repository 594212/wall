use super::Connection;
use crate::models::media::ChunkBy;
use crate::models::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

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
