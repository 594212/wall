use super::Connection;
use crate::models::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;

pub async fn create_serial(
    conn: &mut Connection,
    title: &str,
    description: &str,
) -> Result<Serial, Error> {
    use crate::schema::serials;

    let serial = NewSerial { title, description };

    let media = diesel::insert_into(serials::table)
        .values(&serial)
        .returning(Serial::as_returning())
        .get_result(conn)
        .await?;
    Ok(media)
}

pub async fn count_serials(conn: &mut Connection) -> Result<i64, Error> {
    use crate::schema::serials;
    serials::table.count().get_result(conn).await
}

pub async fn paging_serials(
    limit: i64,
    page: i64,
    conn: &mut Connection,
) -> Result<Vec<Serial>, Error> {
    use crate::schema::serials;
    let serials: Vec<Serial> = serials::table
        .select(Serial::as_returning())
        .limit(limit)
        .offset(page * limit)
        .load(conn)
        .await?;

    Ok(serials)
}
