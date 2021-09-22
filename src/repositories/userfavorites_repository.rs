use crate::models::users::{Pool, User, UserFavorite};
use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

pub fn query_belong_to_user(user: &User, pool: &web::Data<Pool>) -> Result<Vec<UserFavorite>, Error> {
	use crate::schema::userfavorites::dsl::id;
	let conn: &PgConnection = &pool.get().unwrap();

	let items = UserFavorite::belonging_to(user)
		.order(id.asc())
		.load::<UserFavorite>(conn)?;

	Ok(items)
}

pub fn query_by_project(q_project_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<Vec<UserFavorite>, Error> {
	use crate::schema::userfavorites::dsl::{project_id, userfavorites};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = userfavorites
		.filter(project_id.eq(q_project_id))
		.load::<UserFavorite>(conn)?;

	Ok(items)
}

pub fn add_favorite_project(
	user_id: uuid::Uuid,
	project_id: uuid::Uuid,
	email: String,
	pool: &web::Data<Pool>,
) -> Result<UserFavorite, Error> {
	use crate::schema::userfavorites::dsl::userfavorites;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_favorite = UserFavorite {
		id: uuid::Uuid::new_v4(),
		user_id: user_id,
		project_id: project_id,
		updated_by: email,
	};

	let favorite = diesel::insert_into(userfavorites)
		.values(&new_favorite)
		.get_result::<UserFavorite>(conn)?;

	Ok(favorite)
}

pub fn delete_favorite_project(
	q_user_id: uuid::Uuid,
	q_project_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userfavorites::dsl::*;

	let deleted = diesel::delete(
		userfavorites
			.filter(user_id.eq(q_user_id))
			.filter(project_id.eq(q_project_id)),
	)
	.execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
