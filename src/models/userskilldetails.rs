use super::super::schema::*;
use crate::models::users::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Insertable, Identifiable)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "userskilldetails"]
#[primary_key(idx)]
pub struct UserSkillDetail {
	pub idx: i32,
	pub user_id: uuid::Uuid,
	pub skill_label: String,
	pub level_index: Option<i32>,
	pub level_label: Option<String>,
	pub years: Option<f64>,
}
