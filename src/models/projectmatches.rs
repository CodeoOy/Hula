use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use crate::models::projects::*;
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Insertable, Identifiable)]
#[belongs_to(Project, foreign_key = "project_id")]
#[table_name = "projectmatches"]
#[primary_key(idx)]
pub struct ProjectMatch {
	pub idx: i32,
	pub project_id: uuid::Uuid,
	pub skill_label: String,
	pub skill_mandatory: bool,
	pub pn_id: uuid::Uuid,
	pub pn_label: String,
	pub required_load: Option<i32>,
	pub required_index: Option<i32>,
	pub required_minyears: Option<f64>,
	pub required_maxyears: Option<f64>,
	pub user_id: uuid::Uuid,
	pub user_first_name: String,
	pub user_last_name: String,
	pub user_is_hidden: bool,
	pub user_load: i32,
	pub user_index: Option<i32>,
	pub user_years: Option<f64>,
}
