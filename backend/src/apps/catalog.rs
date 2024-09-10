use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{db::*, models::*};

#[derive(Deserialize, Serialize)]
struct Page {
    limit: i64,
    page: i64,
}

struct Catalog {
    id: i32,
    name: String,
    categories: Vec<Category>,
    avatar: Media,
}

struct CatalogResponse {
    data: Vec<Catalog>,
    page: i32,
    limit: i32,
    coumt: i32,
}

#[get("/catalog")]
pub async fn catalog(page: web::Query<Page>, pool: web::Data<PgPool>) -> impl Responder {
    let conn = &mut pool
        .get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    paging_serials(page.limit, page.page, conn)
        .and_then(|serials| retrieve_medias(serials, CollectionType::Avatar, conn))
        .map(|medias| HttpResponse::Ok().json(medias))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}
