use actix_web::web;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::users::{Pool, Session};
use crate::errors::ServiceError;

pub fn query_create_session(
	user_id: uuid::Uuid,
	user_email: String,
	pool: web::Data<Pool>,
) -> Result<Session, ServiceError> {
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
		user_id: user_id,
		expire_at: expiration.naive_utc(),
		updated_by: user_email,
	};
	let _rows_inserted = diesel::insert_into(sessions)
		.values(&session)
		.get_result::<Session>(conn)?;

	Ok(session.into())
}

pub fn query_delete_session(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::sessions::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(sessions.filter(user_id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
