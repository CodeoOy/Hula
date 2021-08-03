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
	pub password_plain: Option<String>,
	pub first_name: String,
	pub last_name: String,
	pub expires_at: chrono::NaiveDateTime,
	pub password_pending: bool,
	pub updated_by: String,
}

pub struct ResetPasswordEmail {
	pub id: uuid::Uuid,
	pub email: String,
	pub expires_at: chrono::NaiveDateTime,
	pub password_pending: bool,
	pub updated_by: String,
}

impl Invitation {
	pub fn from_details<S: Into<String>, T: Into<String>, U: Into<String>>(
		email: S,
		password_plain: Option<String>,
		first_name: T,
		last_name: U,
		password_pending: bool,
	) -> Self {
		let emailstr: String = email.into();
		Invitation {
			id: uuid::Uuid::new_v4(),
			email: String::from(&emailstr),
			password_plain: password_plain,
			first_name: first_name.into(),
			last_name: last_name.into(),
			password_pending: password_pending,
			expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
			updated_by: emailstr,
		}
	}
}

impl ResetPasswordEmail {
	pub fn from_details<S: Into<String>>(
		email: S,
		password_pending: bool,
	) -> Self {
		let emailstr: String = email.into();
		ResetPasswordEmail {
			id: uuid::Uuid::new_v4(),
			email: String::from(&emailstr),
			password_pending: true,
			expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
			updated_by: emailstr,
		}
	}
}
