mod apps;
mod db;
mod error;
mod models;
mod schema;
use actix_web::{middleware::Logger, web, App, HttpServer};
use apps::catalog::*;
use db::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let pg_pool = web::Data::new(init_pool());
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(pg_pool.clone())
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(catalog);
}
