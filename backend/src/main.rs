mod apps;
mod db;
mod error;
mod models;
mod schema;
use actix_web::{
    get, guard, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use apps::catalog::catalog;
use db::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let pg_pool = web::Data::new(init_pool());

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(pg_pool.clone())
            .service(web::scope("path").service(hello))
            .service(echo)
            .service(catalog)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/")
                    .guard(guard::Host("www.localhost"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.localhost"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("users") })),
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
