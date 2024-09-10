use crate::error::Error;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{db::*, models::*};

#[derive(Deserialize, Serialize)]
struct Page {
    limit: i64,
    page: i64,
}

struct SerialResponseInner {
    id: i32,
    name: String,
    categories: Vec<Category>,
    avatar: Media,
}

struct SerialResponse {
    data: Vec<SerialResponseInner>,
    page: i32,
    limit: i32,
    count: i32,
}

#[get("/catalog")]
pub async fn catalog(
    page: web::Query<Page>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let conn = &mut pool.get()?;

    Ok(paging_serials(page.limit, page.page, conn)
        .and_then(|serials| retrieve_medias(serials, CollectionType::Avatar, conn))
        .map(|medias| HttpResponse::Ok().json(medias))?)
}

#[get("/")]
pub async fn error() -> impl Responder {
    let error = Error::NotFound(json!({"error": "record not found"}));
    Err(error)?;
    Ok::<_, Error>(HttpResponse::Ok().body("error"))
}
