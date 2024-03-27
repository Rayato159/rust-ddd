use std::time::SystemTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub picture: Option<String>,
    pub price: f64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}