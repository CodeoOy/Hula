use crate::models::users::{Pool, UserFavorite};
use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;
use diesel::result::Error::NotFound;

pub fn add_favorite_project(
	uuid_data: uuid::Uuid,
	project_id: uuid::Uuid,
	email: String,
	pool: &web::Data<Pool>,
) -> Result<UserFavorite, Error> {
	use crate::schema::userfavorites::dsl::userfavorites;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_favorite = UserFavorite {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_data,
		project_id: project_id,
		updated_by: email,
	};

	let favorite = diesel::insert_into(userfavorites)
		.values(&new_favorite)
		.get_result::<UserFavorite>(conn)?;

	Ok(favorite)
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
