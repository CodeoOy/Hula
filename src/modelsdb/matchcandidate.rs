use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "matchcandidates"]
pub struct MatchCandidate {
	pub projectskillid: uuid::Uuid,
	pub userskillid: uuid::Uuid,
	pub projectname: String,
	pub skillname: String,
	pub required_level: String,
	pub required_index: i32,
	pub required_minyears: Option<f32>,
	pub required_maxyears: Option<f32>,
	pub firstname: String,
	pub lastname: String,
	pub user_level: String,
	pub user_index: i32,
	pub user_years: Option<f32>,
}
