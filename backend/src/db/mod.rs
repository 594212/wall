pub mod serial;
pub use serial::*;
pub mod catalog;
pub use catalog::*;
pub mod media;
pub use media::*;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use dotenvy::dotenv;
use std::env;

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
