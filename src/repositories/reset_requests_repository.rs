use actix_web::web;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::invitations::ResetPasswordRequest;
use crate::models::users::Pool;
use diesel::result::Error;

pub fn create_reset_request(q_email: String, pool: &web::Data<Pool>) -> Result<ResetPasswordRequest, Error> {
	use crate::schema::reset_requests::dsl::reset_requests;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_reset_request = ResetPasswordRequest::from_details(q_email);

	let reset_request = diesel::insert_into(reset_requests)
		.values(&new_reset_request)
		.get_result::<ResetPasswordRequest>(conn)?;

	Ok(reset_request)
}
/*
pub fn get_by_reset_request(
	q_reset_request_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<ResetPasswordRequest, Error> {
	use crate::schema::reset_requests::dsl::{email, id, reset_requests};
	let conn: &PgConnection = &pool.get().unwrap();

	let reset_request = reset_requests
		.filter(id.eq(&q_reset_request_id))
		.filter(email.eq(&q_email))
		.get_result::<ResetPasswordRequest>(conn)?;

	Ok(reset_request)
}
*/
