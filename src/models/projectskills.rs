use super::super::schema::*;
use serde::{Deserialize, Serialize};
use crate::models::projects::Project;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Insertable, Identifiable)]
#[belongs_to(Project, foreign_key = "project_id")]
#[table_name = "projectskills"]
#[primary_key(idx)]
pub struct ProjectSkill {
	pub idx: i32,
	pub project_id: uuid::Uuid,
	pub skill_label: String,
	pub pn_id: uuid::Uuid,
	pub required_index: Option<i32>,
	pub required_minyears: Option<f64>,
	pub required_maxyears: Option<f64>,
}
