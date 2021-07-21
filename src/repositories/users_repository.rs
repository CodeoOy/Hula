use crate::models::users::{Pool, User};
use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};
use Error::NotFound;

pub fn query_all(pool: &web::Data<Pool>) -> Result<Vec<User>, Error> {
	use crate::schema::users::dsl::{firstname, lastname, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = users.order((lastname.asc(), firstname.asc())).load::<User>(conn)?;

	Ok(items)
}

pub fn get_by_email(q_email: String, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = users.filter(email.eq(&q_email)).get_result::<User>(conn)?;

	Ok(user)
}

pub fn get(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = users.filter(id.eq(&q_id)).get_result::<User>(conn)?;

	Ok(user)
}

pub fn create(
	q_email: String,
	q_password: String,
	q_first_name: String,
	q_last_name: String,
	pool: &web::Data<Pool>,
) -> Result<User, Error> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_user = User::from_details(q_email, q_password, q_first_name, q_last_name);

	let user: User = diesel::insert_into(users).values(&new_user).get_result(conn)?;

	Ok(user)
}

pub fn update(
	uuid_data: uuid::Uuid,
	q_first_name: String,
	q_last_name: String,
	q_user_is_hidden: bool,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<User, Error> {
	use crate::schema::users::dsl::{firstname, id, is_hidden, lastname, updated_by, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = diesel::update(users)
		.filter(id.eq(uuid_data))
		.set((
			firstname.eq(q_first_name),
			lastname.eq(q_last_name),
			is_hidden.eq(q_user_is_hidden),
			updated_by.eq(q_email),
		))
		.get_result::<User>(conn)?;

	Ok(user)
}

pub fn delete_user(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::users::dsl::id;
	use crate::schema::users::dsl::*;

	let deleted = diesel::delete(users.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
