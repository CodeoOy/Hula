use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::invitations::Pool;
use crate::models::users::User;
use crate::repositories::*;
use crate::utils::hash_password;

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
	pub email: String,
	pub password: String,
	pub id: String,
}

pub async fn register_user(
	user_data: web::Json<UserData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Registering a user: user_data={:#?}", &user_data);
	let res = web::block(move || query(user_data.into_inner(), pool)).await;

	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query(user_data: UserData, pool: web::Data<Pool>) -> Result<User, crate::errors::ServiceError> {
	let invitation_id = uuid::Uuid::parse_str(&user_data.id)?;

	let result = invitations_repository::get_by_invitation(invitation_id.clone(), user_data.email.clone(), &pool);

	if let Ok(invitation) = result {
		let password: String;
		if invitation.password_pending == true {
			password = hash_password(&user_data.password).unwrap();
		} else {
			password = user_data.password;
		}
		if invitation.expires_at > chrono::Local::now().naive_local() {
			let user = users_repository::create(
				invitation.email,
				password,
				invitation.first_name,
				invitation.last_name,
				false,
				&pool,
			)?;

			return Ok(user);
		}
	}

	Err(ServiceError::BadRequest("Invalid Invitation".into()))
}
