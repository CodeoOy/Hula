use crate::models::users::{Pool, UserFavorites};
use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;
use diesel::result::Error::NotFound;

pub fn add_favorite_project(
	uuid_data: uuid::Uuid,
	project_id: uuid::Uuid,
	email: String,
	pool: &web::Data<Pool>,
) -> Result<UserFavorites, Error> {
	use crate::schema::userfavorites::dsl::userfavorites;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_favorite = UserFavorites {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_data,
		project_id: project_id,
		updated_by: email,
	};

	diesel::insert_into(userfavorites)
		.values(&new_favorite)
		.execute(conn)?;

	Ok(new_favorite.into())
}

pub fn delete_favorite_project(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userfavorites::dsl::*;

	let deleted = diesel::delete(userfavorites.filter(id.eq(uuid_data))).execute(conn)?;
	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
