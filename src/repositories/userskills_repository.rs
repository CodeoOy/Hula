use crate::models::users::{Pool, User, UserSkill};
use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;
use diesel::result::Error::NotFound;

pub fn query_belong_to_user(user: &User, pool: &web::Data<Pool>) -> Result<Vec<UserSkill>, Error> {
	let conn: &PgConnection = &pool.get().unwrap();

	let items = UserSkill::belonging_to(user).load::<UserSkill>(conn)?;
	Ok(items)
}

pub fn add_skill(
	uuid_data: uuid::Uuid,
	q_skill_id: uuid::Uuid,
	q_skillscopelevel_id: uuid::Uuid,
	q_years: Option<f64>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<UserSkill, Error> {
	use crate::schema::userskills::dsl::userskills;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_user_skill = UserSkill {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_data,
		skill_id: q_skill_id,
		skillscopelevel_id: q_skillscopelevel_id,
		years: q_years,
		updated_by: q_email,
	};

	diesel::insert_into(userskills)
		.values(&new_user_skill)
		.execute(conn)?;

	Ok(new_user_skill.into())
}

pub fn update_year(
	uuid_data: uuid::Uuid,
	q_user_id: uuid::Uuid,
	q_years: Option<f64>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<UserSkill, Error> {
	use crate::schema::userskills::dsl::*;
	use crate::schema::userskills::dsl::{skill_id, updated_by, years};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = diesel::update(userskills)
		.filter(skill_id.eq(uuid_data))
		.filter(user_id.eq(q_user_id))
		.set((years.eq(q_years), updated_by.eq(q_email)))
		.load::<UserSkill>(conn)?;

	if let Some(user_res) = items.pop() {
		return Ok(user_res.into());
	}
	Err(NotFound)
}

pub fn delete_userskill(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userskills::dsl::{id, userskills};

	let deleted = diesel::delete(userskills.filter(id.eq(uuid_data))).execute(conn)?;
	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}

