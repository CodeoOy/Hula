use crate::errors::ServiceError;
use crate::models::users::{Pool, User};
use actix_web::web;
use diesel::{prelude::*, PgConnection};

pub fn query_all(pool: &web::Data<Pool>) -> Result<Vec<User>, ServiceError> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = users.load::<User>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn get_by_email(q_email: String, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = users.filter(email.eq(&q_email)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		return Ok(user);
	}
	Err(ServiceError::Empty)
}

pub fn get(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = users.filter(id.eq(&q_id)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		return Ok(user);
	}
	Err(ServiceError::Empty)
}

pub fn create(q_email: String, q_password: String, q_first_name: String, q_last_name: String, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();

	let user =
		User::from_details(q_email, q_password, q_first_name, q_last_name);

	let inserted_user: User = diesel::insert_into(users).values(&user).get_result(conn)?;
	return Ok(inserted_user);
}


pub fn query_update(
	uuid_data: String,
	first_name: String,
	last_name: String,
	user_available: bool,
	email: String,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{available, firstname, id, lastname, updated_by, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(users)
		.filter(id.eq(uuid_query))
		.set((
			firstname.eq(first_name),
			lastname.eq(last_name),
			available.eq(user_available),
			updated_by.eq(email),
		))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_delete_user(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::users::dsl::id;
	use crate::schema::users::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(users.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
