use crate::errors::ServiceError;
use crate::models::users::{Pool, UserFavorites};
use actix_web::web;
use diesel::{prelude::*, PgConnection};

pub fn query_add_favorite_project(
	uuid_data: String,
	project_id: uuid::Uuid,
	pool: web::Data<Pool>,
	email: String,
) -> Result<UserFavorites, crate::errors::ServiceError> {
	use crate::schema::userfavorites::dsl::userfavorites;
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let new_favorite = UserFavorites {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_query,
		project_id: project_id,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(userfavorites)
		.values(&new_favorite)
		.get_result::<UserFavorites>(conn);
	if rows_inserted.is_ok() {
		return Ok(new_favorite.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_delete_favorite_project(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userfavorites::dsl::*;
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(userfavorites.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
