use actix_web::web;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::users::Pool;
use crate::models::invitations::Invitation;
use diesel::result::Error;
use diesel::result::Error::NotFound;

pub fn create_invitation(
	q_email: String,
	q_password: String,
	q_first_name: String,
	q_last_name: String,
	pool: &web::Data<Pool>,
) -> Result<Invitation, Error> {
	use crate::schema::invitations::dsl::invitations;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_invitation = Invitation::from_details(q_email, q_password, q_first_name, q_last_name);
	diesel::insert_into(invitations)
		.values(&new_invitation)
		.execute(conn)?;

	Ok(new_invitation)
}

pub fn get_by_invitation(q_invitation_id: uuid::Uuid, q_email: String, pool: &web::Data<Pool>) -> Result<Invitation, Error> {
	use crate::schema::invitations::dsl::{email, id, invitations};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = invitations
		.filter(id.eq(&q_invitation_id))
		.filter(email.eq(&q_email))
		.load::<Invitation>(conn)?;

	if let Some(invitation) = items.pop() {
		return Ok(invitation);
	}
	Err(NotFound)
}


