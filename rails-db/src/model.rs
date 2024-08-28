use diesel::prelude::*;
use serde::Serialize;
use crate::schema::serials::description;

#[derive(Queryable, Serialize, Identifiable, Debug, PartialEq)]
#[diesel(table_name = books)]
pub struct Serial {
    pub id: i32,
    pub name: String,
    pub description: String,
}
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belong_to(Serial))]
#[diesel(table_name = serials)]
pub struct Episode {
    pub id: i32,
    pub name: Option<String>,
    pub number: i32,
    pub serial_id: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    login: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Episode))]
#[diesel(table_name = views)]
#[diesel(primary_key(user_id, episode_id))]
pub struct View {
    pub user_id: i32,
    pub episode_id: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Episode))]
#[diesel(table_name = likes)]
#[diesel(primary_key(user_id, episode_id))]
pub struct Like {
    pub user_id: i32,
    pub episode_id: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[disel(belongs_to(Serial))]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: i32,
    pub number: i32,
    pub user_id: i32,
    pub serial_id: i32,
    pub avrg: f32,
}
