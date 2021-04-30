use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Insertable)]
#[table_name = "users"]
pub struct User {
	pub id: uuid::Uuid,
	pub isadmin: bool,
	pub ispro: bool,
	pub available: bool,
	pub email: String,
	pub firstname: String,
	pub lastname: String,
	pub hash: String,
	pub created_at: chrono::NaiveDateTime,
	pub updated_by: String,
	pub updated_at: chrono::NaiveDateTime,
	pub inserted_by: String,
	pub inserted_at: chrono::NaiveDateTime,
	pub updated_count: i16,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "userskills"]
pub struct Skill {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    skill_id: uuid::Uuid,
	skillscopelevel_id: uuid::Uuid,
	years: Option<f32>,
	updated_by: String,
}

impl User {
	pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
		User {
			id: uuid::Uuid::new_v4(),
			isadmin: false,
			ispro: true,
			available: true,
			email: email.into(),
			firstname: String::from("Pihla"),
			lastname: String::from("Placeholder"),
			hash: pwd.into(),
			created_at: chrono::Local::now().naive_local(),
			updated_by: String::from("email"),
			updated_at: chrono::Local::now().naive_local(),
			inserted_by: String::from("Maija"),
			inserted_at: chrono::Local::now().naive_local(),
			updated_count: 0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
	pub email: String,
	pub uid: uuid::Uuid
}

impl From<User> for SlimUser {
	fn from(user: User) -> Self {
		SlimUser { email: user.email, uid: user.id }
	}
}