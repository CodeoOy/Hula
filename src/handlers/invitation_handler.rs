use actix_web::{error::BlockingError, web, HttpResponse};
use serde::Deserialize;
use log::{debug, trace};

use crate::email_service::send_invitation;
use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, Pool};
use crate::utils::hash_password;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct InvitationData {
	pub email: String,
	pub password_plain: String,
	pub first_name: String,
	pub last_name: String,
}

pub async fn post_invitation(
	invitation_data: web::Json<InvitationData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Posting invitation: invitation_data={:#?}", &invitation_data);
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
	let invitation = dbg!(query(
		invdata.email,
		invdata.password_plain,
		invdata.first_name,
		invdata.last_name,
		pool
	)?);
	send_invitation(&invitation)
}

fn query(
	eml: String,
	psw: String,
	first_name: String,
	last_name: String,
	pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
	let res = users_repository::get_by_email(eml.clone(), &pool);
	let password: String = hash_password(&psw)?;
	match res {
		Ok(user) => {
			debug!("User {} already found. Cannot process invitation.", &user.email);
			return Err(ServiceError::Unauthorized);
		},
		Err(ServiceError::Empty) => {
			let invitation = invitations_repository::query_create_invitation(eml, password, first_name, last_name, &pool)?;
			Ok(invitation)
		},
		Err(error) => {
			Err(error.into())
		}
	}
}
