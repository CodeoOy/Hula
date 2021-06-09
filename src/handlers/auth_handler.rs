use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool, Session, SlimUser, User};
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
	pub email: String,
	pub password: String,
}

pub async fn logout(
	uuid_data: web::Path<String>,
	id: Identity,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_delete_session(uuid_data.into_inner(), pool)).await;
	println!("\nLogout\n");
	id.forget();
	match res {
		Ok(sessio) => Ok(HttpResponse::Ok().json(&sessio)),
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
			let user_string = serde_json::to_string(&user).unwrap();
			//let user_to_vue = user_string.clone();
			id.remember(user_string);
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

fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = users.filter(email.eq(&auth_data.email)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		if let Ok(matching) = verify(&user.hash, &auth_data.password) {
			if matching {
				println!("\nSuccessfully authenticated (db query).\n");
				return Ok(user.into());
			}
		}
	}
	Err(ServiceError::Unauthorized)
}

fn query_create_session(auth_data: AuthData, pool: web::Data<Pool>) -> Result<Session, crate::errors::ServiceError> {
	use crate::schema::sessions::dsl::sessions;
	use crate::schema::users::dsl::{email, id, users};

	let conn: &PgConnection = &pool.get().unwrap();
	let userid = users.select(id).filter(email.eq(&auth_data.email)).get_result(conn)?;
	let new_session = Session {
		id: uuid::Uuid::new_v4(),
		user_id: userid,
		updated_by: auth_data.email,
	};
	let rows_inserted = diesel::insert_into(sessions)
		.values(&new_session)
		.get_result::<Session>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSession added successfully.\n");
		return Ok(new_session.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn new_session(auth_data: web::Json<AuthData>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_create_session(auth_data.into_inner(), pool)).await;
	println!("\nNew session\n");
	match res {
		Ok(session) => Ok(HttpResponse::Ok().json(&session)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_session(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::sessions::dsl::id;
	use crate::schema::sessions::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(sessions.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
