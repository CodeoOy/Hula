use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool, Session, User};
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
	pub email: String,
	pub password: String,
}

pub async fn logout(
	id: Identity,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_delete_session(logged_user.uid.to_string(), pool)).await;
	println!("\nLogout\n");
	id.forget();
	match res {
		Ok(session) => Ok(HttpResponse::Ok().json(&session)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn login(
	auth_data: web::Json<AuthData>,
	id: Identity,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query(auth_data.into_inner(), pool)).await;
	println!("\nAuthenticating....\n");
	match res {
		Ok(user) => {
			id.remember(user.id.to_string());
			println!("\nSuccessfully authenticated (login).\n");
			Ok(HttpResponse::Ok().finish()) // Instead of empty response, do we need the cookie to body in order to call it from Vue?
			                    //Ok(HttpResponse::Ok().json(user_to_vue))
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
	HttpResponse::Ok().json(logged_user.uid)
}

fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<Session, ServiceError> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = users.filter(email.eq(&auth_data.email)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		if let Ok(matching) = verify(&user.hash, &auth_data.password) {
			if matching {
				println!("\nSuccessfully authenticated (db query).\n");

				if let Ok(session) = query_create_session(user.id.clone(), user.email.clone(), pool) {
					return Ok(session);
				}
			}
		}
	}
	Err(ServiceError::Unauthorized)
}

fn query_create_session(
	user_id: uuid::Uuid,
	user_email: String,
	pool: web::Data<Pool>,
) -> Result<Session, crate::errors::ServiceError> {
	use crate::schema::sessions::dsl::sessions;

	let expiry_mins = std::env::var("SESSION_EXPIRY_MINS").unwrap_or_else(|_| "60".to_string());
	let mins = expiry_mins.parse::<i64>().expect(&format!(
		"Invalid number format in SESSION_EXPIRY_MINS: {}",
		expiry_mins
	));

	let expiration = chrono::offset::Utc::now() + chrono::Duration::minutes(mins);

	let conn: &PgConnection = &pool.get().unwrap();

	let session = Session {
		id: uuid::Uuid::new_v4(),
		user_id: user_id,
		expire_at: expiration.naive_utc(),
		updated_by: user_email,
	};
	let rows_inserted = diesel::insert_into(sessions)
		.values(&session)
		.get_result::<Session>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSession added successfully.\n");
		return Ok(session.into());
	}
	Err(ServiceError::Unauthorized)
}

fn query_delete_session(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::sessions::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(sessions.filter(user_id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
