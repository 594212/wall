pub mod models;
mod repository;
pub mod schema;

use std::env;

use crate::models::media::{ModelType, NewMedia};
use diesel::{pg::PgConnection, Connection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::episodes::{NewSerial, Serial};
use models::media::Media;
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_serial(conn: &mut PgConnection, title: &str, description: &str) -> Serial {
    use crate::schema::serials;

    let serial = NewSerial { title, description };

    diesel::insert_into(serials::table)
        .values(&serial)
        .returning(Serial::as_returning())
        .get_result(conn)
        .expect("Error saving media")
}

pub fn create_media(
    conn: &mut PgConnection,
    uuid: Uuid,
    model_id: i64,
    model_type: ModelType,
    file_name: &str,
    mime_type: &str,
    conversion: &str,
    size: i64,
) -> Media {
    use crate::schema::medias;
    let new_media = NewMedia {
        uuid,
        model_id,
        model_type,
        file_name,
        mime_type,
        conversion,
        size,
    };

    diesel::insert_into(medias::table)
        .values(&new_media)
        .returning(Media::as_returning())
        .get_result(conn)
        .expect("Error saving media")
}

pub fn inner_join(conn: &mut PgConnection, model_id: i64, model_type: ModelType) {
    use crate::schema::medias;
    use crate::schema::serials;
    serials.inner_join(medias.on(model_id));
}
