use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::email_service::send_invitation;
use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, Pool};
use crate::models::users::User;
use crate::utils::hash_password;

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
	// run diesel blocking code
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
) -> Result<Invitation, crate::errors::ServiceError> {
	use crate::schema::invitations::dsl::invitations;
	use crate::schema::users::dsl::*;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = users.filter(email.eq(eml.clone())).load::<User>(conn)?;
	let password: String = hash_password(&psw)?;
	if items.is_empty() {
		let new_invitation = Invitation::from_details(eml, password, first_name, last_name);
		let inserted_invitation = diesel::insert_into(invitations)
			.values(&new_invitation)
			.get_result(conn)?;
		Ok(inserted_invitation)
	} else {
		Err(ServiceError::Unauthorized)
	}
}
