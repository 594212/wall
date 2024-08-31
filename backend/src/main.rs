use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use db::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_pool = init_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(web::scope("path").service(hello))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-land.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
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
