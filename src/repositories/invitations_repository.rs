use actix_web::web;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::Error::NotFound;

use crate::models::invitations::Invitation;
use crate::models::users::Pool;
use diesel::result::Error;

pub fn create_invitation(
	q_email: String,
	q_password: Option<String>,
	q_first_name: String,
	q_last_name: String,
	q_password_pending: bool,
	q_reset_request_id: Option<uuid::Uuid>,
	pool: &web::Data<Pool>,
) -> Result<Invitation, Error> {
	use crate::schema::invitations::dsl::invitations;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_invitation = Invitation::from_details(q_email, q_password, q_first_name, q_last_name, q_password_pending, q_reset_request_id);

	let invitation = diesel::insert_into(invitations)
		.values(&new_invitation)
		.get_result::<Invitation>(conn)?;

	Ok(invitation)
}

pub fn get_by_invitation(
	q_invitation_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Invitation, Error> {
	use crate::schema::invitations::dsl::{email, id, invitations};
	let conn: &PgConnection = &pool.get().unwrap();

	let invitation = invitations
		.filter(id.eq(&q_invitation_id))
		.filter(email.eq(&q_email))
		.get_result::<Invitation>(conn)?;

	Ok(invitation)
}

pub fn delete_invitation(	
	q_id: uuid::Uuid,
	pool: &web::Data<Pool>)
-> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::invitations::dsl::*;

	let deleted = diesel::delete(invitations.filter(id.eq(q_id))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
