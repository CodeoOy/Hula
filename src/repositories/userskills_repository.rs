use crate::errors::ServiceError;
use crate::models::users::{Pool, User, UserSkill};
use actix_web::web;
use diesel::{prelude::*, PgConnection};

pub fn query_belong_to_user(user: &User, pool: &web::Data<Pool>) -> Result<Vec<UserSkill>, diesel::result::Error> {
	let conn: &PgConnection = &pool.get().unwrap();

	let items = UserSkill::belonging_to(user).load::<UserSkill>(conn)?;
	Ok(items)
}

pub fn query_add_skill(
	uuid_data: String,
	skill_data: web::Json<UserSkill>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<UserSkill, crate::errors::ServiceError> {
	use crate::schema::userskills::dsl::userskills;
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let new_user_skill = UserSkill {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_query,
		skill_id: skill_data.skill_id,
		skillscopelevel_id: skill_data.skillscopelevel_id,
		years: skill_data.years,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(userskills)
		.values(&new_user_skill)
		.get_result::<UserSkill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		return Ok(new_user_skill.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_update_year(
	uuid_data: String,
	q_user_id: uuid::Uuid,
	q_years: Option<f64>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<UserSkill, crate::errors::ServiceError> {
	use crate::schema::userskills::dsl::*;
	use crate::schema::userskills::dsl::{skill_id, updated_by, years};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(userskills)
		.filter(skill_id.eq(uuid_query))
		.filter(user_id.eq(q_user_id))
		.set((years.eq(q_years), updated_by.eq(email)))
		.load::<UserSkill>(conn)?;
	if let Some(user_res) = items.pop() {
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_delete_userskill(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userskills::dsl::{id, userskills};

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(userskills.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}

