use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projects"]
pub struct Project {
	pub id: uuid::Uuid,
	pub available: bool,
	pub name: String,
	pub updated_by: String,
	pub updated_at: chrono::NaiveDateTime,
	pub inserted_by: String,
	pub inserted_at: chrono::NaiveDateTime,
	pub updated_count: i16,
}