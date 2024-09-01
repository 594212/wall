use diesel::prelude::*;
#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name=crate::schema::scrapers)]
pub struct Scraper {
    pub id: i32,
    pub target_url: String,
}
