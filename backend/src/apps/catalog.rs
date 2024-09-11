use std::{
    collections::{linked_list, HashMap},
    ops::RemAssign,
};

use crate::error::Error;
use actix_web::{get, web, HttpResponse, Responder};
use diesel_async::pooled_connection::PoolError;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    avatar: Media,
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
    let mut medias = retrieve_medias(&serials, CollectionType::Avatar, conn).await?;
    let categories = retrieve_categories(&serials, 20, 0, conn).await?;
    let count = count_serials(conn).await?;

    let inner = itertools::izip!(serials, &mut medias, categories)
        .map(|(s, m, c)| {
            let tuple_categories = category_grouped_by(c);
            SerialResponseInner {
                id: s.id,
                title: s.title,
                avatar: m.remove(0),
                tags: tuple_categories.0,
                genres: tuple_categories.1,
                authors: tuple_categories.2,
                years: tuple_categories.3,
                statuses: tuple_categories.4,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(SerialResponse {
        data: inner,
        page: page.page,
        limit: page.limit,
        count: count,
    }))
}

fn category_grouped_by(
    categories: Vec<Category>,
) -> (
    Vec<CategoryResponse>,
    Vec<CategoryResponse>,
    Vec<CategoryResponse>,
    Vec<CategoryResponse>,
    Vec<CategoryResponse>,
) {
    let mut tuple_categories = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for c in categories.into_iter() {
        match c.category_type {
            CategoryType::Tag => tuple_categories.0.push(c.into()),
            CategoryType::Genre => tuple_categories.1.push(c.into()),
            CategoryType::Author => tuple_categories.2.push(c.into()),
            CategoryType::Year => tuple_categories.3.push(c.into()),
            CategoryType::Status => tuple_categories.4.push(c.into()),
        }
    }
    tuple_categories
}

#[get("/")]
pub async fn error() -> impl Responder {
    let error = Error::NotFound(json!({"error": "record not found"}));
    Err(error)?;
    Ok::<_, Error>(HttpResponse::Ok().body("error"))
}

#[get("/tuple")]
pub async fn tuple() -> impl Responder {
    let tuple = TupleStruct(
        34,
        InnerStruct {
            first: vec!["struct".to_string(), "something".to_string()],
        },
    );
    HttpResponse::Ok().json(OuterStruct { outer: tuple })
}

#[derive(Serialize)]
struct OuterStruct {
    outer: TupleStruct,
}
#[derive(Serialize)]
struct TupleStruct(i64, InnerStruct);
#[derive(Serialize)]
struct InnerStruct {
    first: Vec<String>,
}
