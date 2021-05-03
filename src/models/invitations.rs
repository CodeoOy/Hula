use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
	pub id: uuid::Uuid,
	pub email: String,
	pub password_plain: String,
	pub first_name: String,
	pub last_name: String,
	pub expires_at: chrono::NaiveDateTime,
	pub updated_by: String,
	pub updated_at: chrono::NaiveDateTime,
	pub inserted_by: String,
	pub inserted_at: chrono::NaiveDateTime,
	pub updated_count: i16,
}

impl Invitation {
	pub fn from_details<S: Into<String>, T: Into<String>, U: Into<String>, V: Into<String>>(email: S, password_plain: T, first_name: U, last_name: V) -> Self {
		Invitation {
			id: uuid::Uuid::new_v4(),
			email: email.into(),
			password_plain: password_plain.into(),
			first_name: first_name.into(),
			last_name: last_name.into(),
			expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
			updated_by: String::from("email"),
			updated_at: chrono::Local::now().naive_local(),
			inserted_by: String::from("Maija"),
			inserted_at: chrono::Local::now().naive_local(),
			updated_count: 0,
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
