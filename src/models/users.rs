use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Insertable)]
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
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "userid")]
#[table_name = "userskills"]
pub struct Skill {
    id: uuid::Uuid,
    userid: uuid::Uuid,
    skillid: uuid::Uuid,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserExpanded {
	pub user: User,
	pub skills: Vec<Skill>,
}