use super::super::schema::*;
use crate::errors::ServiceError;
use crate::models;
use crate::repositories::*;
use actix_identity::Identity;
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
use diesel::{r2d2::ConnectionManager, PgConnection};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};
use log::debug;

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
	pub inserted_at: chrono::NaiveDateTime,
	pub updated_by: String,
	pub is_employee: bool,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "userskills"]
pub struct UserSkill {
	pub id: uuid::Uuid,
	pub user_id: uuid::Uuid,
	pub skill_id: uuid::Uuid,
	pub skillscopelevel_id: uuid::Uuid,
	pub years: Option<f64>,
	pub updated_by: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "userfavorites"]
pub struct UserFavorites {
	pub id: uuid::Uuid,
	pub user_id: uuid::Uuid,
	pub project_id: uuid::Uuid,
	pub updated_by: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "sessions"]
pub struct Session {
	pub id: uuid::Uuid,
	pub user_id: uuid::Uuid,
	pub expire_at: chrono::NaiveDateTime,
	pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "activesessions"]
pub struct ActiveSession {
	pub session_id: uuid::Uuid,
	pub user_id: uuid::Uuid,
	pub email: String,
	pub expire_at: chrono::NaiveDateTime,
	pub isadmin: bool,
}

impl User {
	pub fn from_details<S: Into<String>, T: Into<String>, U: Into<String>, V: Into<String>>(
		email: S,
		pwd: T,
		first_name: U,
		last_name: V,
	) -> Self {
		let emailstr: String = email.into();
		User {
			id: uuid::Uuid::new_v4(),
			isadmin: false,
			ispro: true,
			available: true,
			email: String::from(&emailstr),
			firstname: first_name.into(),
			lastname: last_name.into(),
			hash: pwd.into(),
			inserted_at: chrono::Local::now().naive_local(),
			updated_by: emailstr,
			is_employee: false,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedUser {
	pub email: String,
	pub uid: uuid::Uuid,
	pub session_id: uuid::Uuid,
	pub isadmin: bool,
}

impl From<ActiveSession> for LoggedUser {
	fn from(session: ActiveSession) -> Self {
		LoggedUser {
			email: session.email,
			uid: session.user_id,
			session_id: session.session_id,
			isadmin: session.isadmin,
		}
	}
}

impl FromRequest for LoggedUser {
	type Config = ();
	type Error = Error;
	type Future = Ready<Result<LoggedUser, Error>>;

	fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
		if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
			if let Some(cookie) = identity.identity() {
				let pool = req.app_data::<Data<models::users::Pool>>().unwrap().clone();
				let id_res = uuid::Uuid::parse_str(&cookie);
				match id_res {
					Ok(id) => {
						let session = activesessions_repository::get_session_by_id(id, &pool);
						if let Ok(s) = session {
							if s.expire_at > chrono::offset::Utc::now().naive_utc() {
								let u: LoggedUser = s.into();
								return ok(u);
							}

							debug!("Session expired!");
						} else {
							debug!("No active session found!");
						}
					}
					Err(err) => {
						debug!("Not an UUID in the cookie! Error: {:?}", err);
					}
				};
			} else {
				debug!("Identity (cookie) not received!");
			}
		} else {
			debug!("Request processing failed!");
		}

		err(ServiceError::Unauthorized.into())
	}
}
