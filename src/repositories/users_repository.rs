use crate::models::users::{Pool, User};
use Error::NotFound;
use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;

pub fn query_all(pool: &web::Data<Pool>) -> Result<Vec<User>, Error> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = users.load::<User>(conn)?;
	Ok(items)
}

pub fn get_by_email(q_email: String, pool: &web::Data<Pool>) -> Result<Option<User>, Error> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = users.filter(email.eq(&q_email)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		return Ok(Some(user));
	}

	Ok(None)
}

pub fn get(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = users.filter(id.eq(&q_id)).get_result::<User>(conn)?;
	Ok(user)
}

pub fn create(q_email: String, q_password: String, q_first_name: String, q_last_name: String, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();

	let user =
		User::from_details(q_email, q_password, q_first_name, q_last_name);

	let inserted_user: User = diesel::insert_into(users).values(&user).get_result(conn)?;
	return Ok(inserted_user);
}


pub fn update(
	uuid_data: uuid::Uuid,
	q_first_name: String,
	q_last_name: String,
	q_user_available: bool,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<User, Error> {
	use crate::schema::users::dsl::{available, firstname, id, lastname, updated_by, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = diesel::update(users)
		.filter(id.eq(uuid_data))
		.set((
			firstname.eq(q_first_name),
			lastname.eq(q_last_name),
			available.eq(q_user_available),
			updated_by.eq(q_email),
		))
		.load::<User>(conn)?;
	
	if let Some(user_res) = items.pop() {
		return Ok(user_res.into());
	}
	Err(NotFound)
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
