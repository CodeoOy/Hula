use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
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

/*
impl Skill {
	pub fn from_details<S: Into<String>, T: Into<String>>(
		email: S,
		label: T,
	) -> Self {
		Skill {
			id: uuid::Uuid::new_v4(),
			label: label.into(),
			skillcategory_id: uuid::Uuid::new_v4(),
			skillscope_id: uuid::Uuid::new_v4(),
			updated_by: email.into(),
		}
	}
}
*/