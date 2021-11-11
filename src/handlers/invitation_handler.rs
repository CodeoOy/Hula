use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::result::Error::NotFound;
use log::{debug, trace};
use serde::Deserialize;

use crate::email_service::{send_invitation, send_reset_request};
use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, Pool, ResetPasswordRequest};
use crate::models::users::{LoggedUser};
use crate::repositories::*;
use crate::utils::hash_password;

#[derive(Deserialize, Debug)]
pub struct InvitationData {
	pub email: String,
	pub password_plain: Option<String>,
	pub first_name: String,
	pub last_name: String,
	pub password_pending: bool,
}

#[derive(Deserialize, Debug)]
pub struct ResetRequestData {
	pub email: String,
}

pub async fn post_invitation(
	invitation_data: web::Json<InvitationData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Posting invitation: invitation_data = {} {} {}",
		&invitation_data.email,
		&invitation_data.first_name,
		&invitation_data.last_name
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || create_invitation(invitation_data.into_inner(), pool)).await;

	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn create_invitation(invdata: InvitationData, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let invitation = query_invitation(
		invdata.email,
		invdata.password_plain,
		invdata.first_name,
		invdata.last_name,
		invdata.password_pending,
		pool,
	)?;
	send_invitation(&invitation)
}

pub async fn post_reset_request(
	reset_request_data: web::Json<ResetRequestData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Posting reset_request: reset_request_data = {}",
		&reset_request_data.email
	);
	let res = web::block(move || create_reset_request(reset_request_data.into_inner(), pool)).await;

	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn create_reset_request(invdata: ResetRequestData, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let reset_request = query_reset_request(invdata.email, pool)?;
	send_reset_request(&reset_request)
}

fn query_invitation(
	eml: String,
	psw: Option<String>,
	first_name: String,
	last_name: String,
	password_pending: bool,
	pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
	let res = users_repository::get_by_email(eml.clone(), &pool);
	let password_hashed = psw.map(|s| hash_password(&s).unwrap());
	match res {
		Ok(user) => {
			debug!("User {} already found. Cannot process invitation.", &user.email);
			return Err(ServiceError::Unauthorized);
		}
		Err(NotFound) => {
			let mut reset_request_id: Option<uuid::Uuid> = None;

			if password_pending {
				let reset_request = reset_requests_repository::create_reset_request(eml.clone(), &pool)?;
				reset_request_id = Some(reset_request.id);
			}

			let invitation = invitations_repository::create_invitation(
				eml,
				password_hashed,
				first_name,
				last_name,
				password_pending,
				reset_request_id,
				&pool,
			)?;

			Ok(invitation)
		}
		Err(error) => Err(error.into()),
	}
}

fn query_reset_request(eml: String, pool: web::Data<Pool>) -> Result<ResetPasswordRequest, ServiceError> {
	let res = users_repository::get_by_email(eml.clone(), &pool);
	match res {
		Ok(_) => {
			let reset_request = reset_requests_repository::create_reset_request(eml, &pool)?;
			Ok(reset_request)
		}
		Err(NotFound) => {
			debug!("User ({}) not found. Cannot process reset request.", eml.clone());
			return Err(ServiceError::Unauthorized);
		}
		Err(error) => Err(error.into()),
	}
}
