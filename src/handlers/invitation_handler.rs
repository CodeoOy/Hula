use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::result::Error::NotFound;
use log::{debug, trace};
use serde::Deserialize;

use crate::email_service::send_invitation;
use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, Pool};
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

pub async fn post_invitation(
	invitation_data: web::Json<InvitationData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Posting invitation: invitation_data = {} {} {}",
		&invitation_data.email,
		&invitation_data.first_name,
		&invitation_data.last_name
	);
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
	let invitation = query(
		invdata.email,
		invdata.password_plain,
		invdata.first_name,
		invdata.last_name,
		invdata.password_pending,
		pool,
	)?;
	send_invitation(&invitation)
}

fn query(
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
			let invitation = invitations_repository::create_invitation(eml, password_hashed, first_name, last_name, password_pending, &pool)?;
			Ok(invitation)
		}
		Err(error) => Err(error.into()),
	}
}
