use crate::models::episodes::Episode;
use crate::models::episodes::Serial;
use crate::models::user::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Episode))]
#[diesel(table_name = crate::schema::views)]
#[diesel(primary_key(user_id, episode_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct View {
    pub user_id: i32,
    pub episode_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Episode))]
#[diesel(table_name = crate::schema::likes)]
#[diesel(primary_key(user_id, episode_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Like {
    pub user_id: i32,
    pub episode_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Serial))]
#[diesel(table_name = crate::schema::ratings)]
#[diesel(primary_key(user_id, serial_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rating {
    pub number: i16,
    pub user_id: i32,
    pub serial_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
