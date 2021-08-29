use crate::models::users::{Pool, UserUploads};
use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};
use std::fs;

pub fn create_file(
	q_user_id: uuid::Uuid,
	q_filename: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<UserUploads, Error> {
	use crate::schema::useruploads::dsl::useruploads;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_file = UserUploads {
		id: uuid::Uuid::new_v4(),
		user_id: q_user_id,
		filename: q_filename,
		updated_by: q_email,
	};

	let file = diesel::insert_into(useruploads)
		.values(&new_file)
		.get_result::<UserUploads>(conn)?;

	Ok(file)
}

pub fn delete_file(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();

	use crate::schema::useruploads::dsl::id;
	use crate::schema::useruploads::dsl::*;

	let deleted = diesel::delete(useruploads.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		let removed = delete_physical_file(uuid_data);
		if removed.is_err() {
			return Err(NotFound);
		} else {
			return Ok(());
		}
	}
	Err(NotFound)
}

fn delete_physical_file(filename: uuid::Uuid) -> std::io::Result<()> {
	let filename = format!("./{}.pdf", filename);
	fs::remove_file(filename)?;
	Ok(())
}

pub fn get_by_userid(id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<Vec<UserUploads>, Error> {
	use crate::schema::useruploads::dsl::{user_id, useruploads};
	let conn: &PgConnection = &pool.get().unwrap();

	let files = useruploads.filter(user_id.eq(&id)).load::<UserUploads>(conn)?;

	Ok(files)
}
