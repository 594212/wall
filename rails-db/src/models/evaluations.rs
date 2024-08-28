use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::schema::views;
use crate::schema::likes;
use crate::schema::ratings;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Episode))]
#[diesel(table_name = views)]
#[diesel(primary_key(user_id, episode_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct View {
    pub user_id: i32,
    pub episode_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Episode))]
#[diesel(table_name = likes)]
#[diesel(primary_key(user_id, episode_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Like {
    pub user_id: i32,
    pub episode_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Serial))]
#[diesel(table_name = ratings)]
#[diesel(primary_key(user_id, serial_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rating {
    pub number: i32,
    pub user_id: i32,
    pub serial_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
