use actix_web::web;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::users::{Pool, Session};
use diesel::result::Error;

pub fn create_session(
	q_user_id: uuid::Uuid,
	q_user_email: String,
	pool: &web::Data<Pool>,
) -> Result<Session, Error> {
	use crate::schema::sessions::dsl::sessions;

	let expiry_mins = std::env::var("SESSION_EXPIRY_MINS").unwrap_or_else(|_| "60".to_string());
	let mins = expiry_mins.parse::<i64>().expect(&format!(
		"Invalid number format in SESSION_EXPIRY_MINS: {}",
		expiry_mins
	));

	let expiration = chrono::offset::Utc::now() + chrono::Duration::minutes(mins);

	let conn: &PgConnection = &pool.get().unwrap();

	let session = Session {
		id: uuid::Uuid::new_v4(),
		user_id: q_user_id,
		expire_at: expiration.naive_utc(),
		updated_by: q_user_email,
	};

	diesel::insert_into(sessions)
		.values(&session)
		.execute(conn)?;

	Ok(session.into())
}

pub fn delete_session(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<usize, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::sessions::dsl::*;

	let deleted = diesel::delete(sessions.filter(user_id.eq(uuid_data))).execute(conn)?;
	Ok(deleted)
}
