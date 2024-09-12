use super::Connection;
use crate::models::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;

pub async fn retrieve_categories(
    serials: &Vec<Serial>,
    limit: i64,
    offset: i64,
    conn: &mut Connection,
) -> Result<Vec<Vec<Category>>, Error> {
    use crate::schema::categories;

    let categories = CategorySerial::belonging_to(serials)
        .inner_join(categories::table)
        .limit(limit)
        .offset(offset)
        .select((CategorySerial::as_select(), Category::as_select()))
        .load(conn)
        .await?; // category_id category_id, ...

    Ok(categories
        .grouped_by(serials)
        .into_iter()
        .map(|c| c.into_iter().map(|(_, category)| category).collect())
        .collect())
}
pub async fn retrieve_category(
    limit: i64,
    offset: i64,
    serials: &Vec<Serial>,
    category_type: CategoryType,
    conn: &mut Connection,
) -> Result<Vec<impl Iterator<Item = Category>>, Error> {
    use crate::schema::categories;

    let categories = CategorySerial::belonging_to(serials)
        .inner_join(categories::table)
        .filter(categories::category_type.eq(category_type))
        .limit(limit)
        .offset(offset)
        .select((CategorySerial::as_select(), Category::as_select()))
        .load(conn)
        .await?; // category_id category_id, ...

    Ok(categories
        .grouped_by(serials)
        .into_iter()
        .map(|c| c.into_iter().map(|(_, category)| category))
        .collect())
}
