use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projectskills"]
pub struct Projectskills {
	pub project_id: uuid::Uuid,
	pub skill_label: String,
	pub pn_id: uuid::Uuid,
	pub required_index: i32,
	pub required_minyears: Option<f64>,
	pub required_maxyears: Option<f64>,
}
