use super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projects"]
pub struct Project {
	pub pid: uuid::Uuid,
	pub available: bool,
	pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
	pub uid: uuid::Uuid,
	pub isadmin: bool,
	pub ispro: bool,
	pub available: bool,
	pub email: String,
	pub firstname: String,
	pub lastname: String,
	pub hash: String,
	pub created_at: chrono::NaiveDateTime,
}

impl User {
	pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
		User {
			uid: uuid::Uuid::new_v4(),
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

// We might need a generic struct to handle more complex queries

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
	pub id: uuid::Uuid,
	pub email: String,
	pub password_plain: String,
	pub expires_at: chrono::NaiveDateTime,
}

impl Invitation {
	pub fn from_details<S: Into<String>, T: Into<String>>(email: S, password_plain: T) -> Self {
		Invitation {
			id: uuid::Uuid::new_v4(),
			email: email.into(),
			password_plain: password_plain.into(),
			expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
		}
	}
}

// any type that implements Into<String> can be used to create Invitation
// impl<T> From<T> for Invitation
// where
//     T: Into<String>,
// {
//     fn from(email: T) -> Self {
//         Invitation {
//             id: uuid::Uuid::new_v4(),
//             email: email.into(),
// 			password_plain: password_plain.into(),
//             expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
	pub email: String,
	pub uid: uuid::Uuid
}

impl From<User> for SlimUser {
	fn from(user: User) -> Self {
		SlimUser { email: user.email, uid: user.uid }
	}
}
