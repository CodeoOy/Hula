use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::users::{Pool, SlimUser, User, LoggedUser};
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
	pub email: String,
	pub password: String,
}

pub async fn logout(id: Identity) -> HttpResponse {
	id.forget();
	HttpResponse::Ok().finish()
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
/// Diesel query
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
