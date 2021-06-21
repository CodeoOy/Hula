use super::super::schema::*;
use crate::errors::ServiceError;
use actix_identity::Identity;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, web::Data};
use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};
use crate::models;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedAdminUser {
	pub email: String,
	pub uid: uuid::Uuid,
	pub session_id: uuid::Uuid,
}

impl From<ActiveSession> for LoggedAdminUser {
	fn from(session: ActiveSession) -> Self {
		if session.isadmin == false {
			panic!("Tried to make a LoggedAdminUser from a normal user");
		}

		LoggedAdminUser {
			email: session.email,
			uid: session.user_id,
			session_id: session.session_id,
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
				if let Some(session) = get_active_session(req, cookie) {
					let u: LoggedUser = session.into();
					return ok(u);
				}
			}
			else {
				println!("extractor: Identity (cookie) not received!");
			}
		}
		else {
			println!("extractor: Request processing failed!");
		}

		err(ServiceError::Unauthorized.into())
	}
}

impl FromRequest for LoggedAdminUser {
	type Config = ();
	type Error = Error;
	type Future = Ready<Result<LoggedAdminUser, Error>>;

	fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
		if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
			if let Some(cookie) = identity.identity() {
				if let Some(session) = get_active_session(req, cookie) {
					if session.isadmin == true {
						let u: LoggedAdminUser = session.into();
						return ok(u);
					}
					else {
						println!("extractor: Not an admin user!");
					}
				}
			}
			else {
				println!("extractor: Identity (cookie) not received!");
			}
		}
		else {
			println!("extractor: Request processing failed!");
		}

		err(ServiceError::Unauthorized.into())
	}
}

fn get_active_session (
	req: &HttpRequest,
	cookie: String,
) -> Option<ActiveSession> {
	let pool = req.app_data::<Data<models::users::Pool>>().unwrap().clone();

	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::activesessions::dsl::session_id;
	use crate::schema::activesessions::dsl::*;

	let id_res = uuid::Uuid::parse_str(&cookie);
	match id_res {
		Ok(id) => {
			let session = activesessions
				.filter(session_id.eq(&id))
				.get_result::<ActiveSession>(conn);

			if let Ok(s) = session {
				if s.expire_at > chrono::offset::Utc::now().naive_utc() {
					return Some(s);
				}

				println!("extractor: Session expired!");
			}
			else {
				println!("extractor: No active session found!");
			}
		},
		Err(err) => {
			println!("extractor: Not an UUID in the cookie! Error: {:?}", err);
		}
	};

	None
}
