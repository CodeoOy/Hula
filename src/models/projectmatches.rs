use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projectmatches"]
pub struct Projectmatches {
	pub project_id: uuid::Uuid,
	pub skill_label: String,
	pub pn_id: uuid::Uuid,
	pub required_index: i32,
	pub required_minyears: Option<f64>,
	pub required_maxyears: Option<f64>,
	pub user_id: uuid::Uuid,
	pub user_first_name: String,
	pub user_last_name: String,
	pub user_is_hidden: bool,
	pub user_load: i32,
	pub user_index: i32,
	pub user_years: Option<f64>,
}
