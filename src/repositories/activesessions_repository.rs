use actix_web::web;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::users::{ActiveSession, Pool};
use diesel::result::Error;

pub fn get_session_by_id(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<ActiveSession, Error> {
	use crate::schema::activesessions::dsl::session_id;
	use crate::schema::activesessions::dsl::*;

	let conn: &PgConnection = &pool.get().unwrap();

	let session = activesessions
		.filter(session_id.eq(&q_id))
		.get_result::<ActiveSession>(conn)?;

	Ok(session)
}
