pub mod schema;
pub mod model;
pub mod media;
pub mod category;
pub mod comment;
mod repository;
pub mod evaluations;
pub mod user;

use std::env;

use diesel::{pg::PgConnection, Connection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::{NewPost, Post};
use crate::media::Media;
use crate::model::{NewSerial, Serial};
use crate::schema::serials;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

fn create_media(conn: &mut PgConnection, name: &str, description: &str) -> Media {
    use crate::schema::serials;
    let serial = NewSerial {
        name,
        description,
    };

    diesel::insert_into(serials::table)
        .values(&serial)
        .returning(Serial::as_returning())
        .get_result(conn)
        .expect("Error saving media")
}
