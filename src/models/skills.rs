use super::super::schema::*;
use diesel::{PgConnection, r2d2::ConnectionManager};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[table_name = "skills"]
pub struct Skill {
	pub id: uuid::Uuid,
	pub label: String,
	pub skillcategory_id: uuid::Uuid,
	pub skillscope_id: uuid::Uuid,
	pub updated_by: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[table_name = "skillscopes"]
pub struct SkillScope {
	pub id: uuid::Uuid,
	pub label: String,
	pub updated_by: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[table_name = "skillcategories"]
pub struct SkillCategory {
	pub id: uuid::Uuid,
	pub label: String,
	pub parent_id: Option<uuid::Uuid>,
	pub updated_by: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[table_name = "skillscopelevels"]
pub struct SkillScopeLevel {
	pub id: uuid::Uuid,
	pub label: String,
	pub skillscope_id: uuid::Uuid,
	pub index: i32,
	pub percentage: Option<i32>,
	pub updated_by: String,
}