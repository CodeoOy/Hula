use crate::models::users::{Pool, User, UserReservation};
use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

pub fn query_belong_to_user(user: &User, pool: &web::Data<Pool>) -> Result<Vec<UserReservation>, Error> {
	use crate::schema::userreservations::dsl::id;
	let conn: &PgConnection = &pool.get().unwrap();

	let items = UserReservation::belonging_to(user)
		.order(id.asc())
		.load::<UserReservation>(conn)?;

	Ok(items)
}

pub fn add_skill(
	uuid_data: uuid::Uuid,
	q_description: String,
	q_begin_time: Option<chrono::NaiveDate>,
	q_end_time: Option<chrono::NaiveDate>,
	q_percentage: Option<i32>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<UserReservation, Error> {
	use crate::schema::userreservations::dsl::userreservations;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_user_reservation = UserReservation {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_data,
		description: q_description,
		begin_time: q_begin_time,
		end_time: q_end_time,
		percentage: q_percentage,
		updated_by: q_email,
	};

	let user_reservation = diesel::insert_into(userreservations)
		.values(&new_user_reservation)
		.get_result::<UserReservation>(conn)?;

	Ok(user_reservation)
}

pub fn update_reservation(
	q_uuid_data: uuid::Uuid,
	q_description: String,
	q_begin_time: Option<chrono::NaiveDate>,
	q_end_time: Option<chrono::NaiveDate>,
	q_percentage: Option<i32>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<UserReservation, Error> {
	use crate::schema::userreservations::dsl::*;
	use crate::schema::userreservations::dsl::{id, percentage, updated_by};
	let conn: &PgConnection = &pool.get().unwrap();

	let user_reservation = diesel::update(userreservations)
		.filter(id.eq(q_uuid_data))
		.set((
			percentage.eq(q_percentage),
			begin_time.eq(q_begin_time),
			end_time.eq(q_end_time),
			description.eq(q_description),
			updated_by.eq(q_email),
		))
		.get_result::<UserReservation>(conn)?;

	Ok(user_reservation)
}

pub fn delete_reservation(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userreservations::dsl::{id, userreservations};

	let deleted = diesel::delete(userreservations.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
