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
}
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projectneedskills"]
pub struct ProjectSkill {
	pub id: uuid::Uuid,
	pub projectneed_id: uuid::Uuid,
	pub skill_id: uuid::Uuid,
	pub skillscopelevel_id: Option<uuid::Uuid>,
	pub min_years: Option<f64>,
	pub max_years: Option<f64>,
	pub updated_by: String,
}
