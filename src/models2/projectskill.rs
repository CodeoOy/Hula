/* pub mod projectskill; */

use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projectskills"]
pub struct ProjectSkill {
	pub id: uuid::Uuid,
	pub projectid: uuid::Uuid,
	pub skillscopelevelid: Option<uuid::Uuid>,
}
	

	