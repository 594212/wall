use crate::error::Error;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{db::*, models::*};

#[derive(Deserialize, Serialize)]
struct Page {
    limit: i64,
    page: i64,
}

#[derive(Serialize)]
struct SerialResponseInner {
    id: i32,
    title: String,
    avatar: String,
    tags: Vec<CategoryResponse>,
    genres: Vec<CategoryResponse>,
    authors: Vec<CategoryResponse>,
    years: Vec<CategoryResponse>,
    statuses: Vec<CategoryResponse>,
}

#[derive(Serialize)]
struct CategoryResponse {
    id: i32,
    name: String,
}

impl From<Category> for CategoryResponse {
    fn from(value: Category) -> Self {
        CategoryResponse {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Serialize)]
struct SerialResponse {
    data: Vec<SerialResponseInner>,
    page: i64,
    limit: i64,
    count: i64,
}

#[get("/catalog")]
pub async fn catalog(
    page: web::Query<Page>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let conn = &mut pool.get().await?;

    let serials = paging_serials(page.limit, page.page, conn).await?;
    let medias = retrieve_medias(&serials, CollectionType::Avatar, conn).await?;
    let tags = retrieve_category(20, 0, &serials, CategoryType::Tag, conn).await?;
    let genres = retrieve_category(20, 0, &serials, CategoryType::Genre, conn).await?;
    let authors = retrieve_category(20, 0, &serials, CategoryType::Author, conn).await?;
    let years = retrieve_category(20, 0, &serials, CategoryType::Year, conn).await?;
    let statuses = retrieve_category(20, 0, &serials, CategoryType::Status, conn).await?;
    let count = count_serials(conn).await?;

    let data = itertools::izip!(serials, medias, tags, genres, years, authors, statuses)
        .map(
            |(serial, media, tags, genres, authors, years, statuses)| SerialResponseInner {
                id: serial.id,
                title: serial.title,
                avatar: media
                    .first()
                    .map_or_else(|| Media::get_default(), |m| m.to_path()),
                tags: tags.map(|c| c.into()).collect(),
                genres: genres.map(|c| c.into()).collect(),
                authors: authors.map(|c| c.into()).collect(),
                years: years.map(|c| c.into()).collect(),
                statuses: statuses.map(|c| c.into()).collect(),
            },
        )
        .collect();

    Ok(HttpResponse::Ok().json(SerialResponse {
        data,
        page: page.page,
        limit: page.limit,
        count,
    }))
}
