use super::super::schema::*;
use diesel::{PgConnection, dsl::Nullable, r2d2::ConnectionManager};
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
	id: uuid::Uuid,
	label: String,
	parent_id: Option<uuid::Uuid>,
	updated_by: String,
}