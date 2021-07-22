use crate::models::skills::{Pool, Skill};
use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

pub fn query_all_skills(pool: &web::Data<Pool>) -> Result<Vec<Skill>, Error> {
	use crate::schema::skills::dsl::{label, skills};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = skills.order(label.asc()).load::<Skill>(conn)?;

	Ok(items)
}

pub fn create_skill(
	q_label: String,
	q_skillcategory_id: uuid::Uuid,
	q_skillscope_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Skill, Error> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_skill = Skill {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		skillcategory_id: q_skillcategory_id,
		skillscope_id: q_skillscope_id,
		updated_by: q_email,
	};

	let skill = diesel::insert_into(skills)
		.values(&new_skill)
		.get_result::<Skill>(conn)?;

	Ok(skill)
}

pub fn update_skill(
	uuid_data: uuid::Uuid,
	q_label: String,
	q_skillcategory_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Skill, Error> {
	use crate::schema::skills::dsl::{skills, *};
	let conn: &PgConnection = &pool.get().unwrap();

	let skill = diesel::update(skills)
		.filter(id.eq(uuid_data))
		.set((
			label.eq(q_label),
			skillcategory_id.eq(q_skillcategory_id),
			updated_by.eq(q_email.clone()),
		))
		.get_result::<Skill>(conn)?;

	Ok(skill)
}

pub fn delete_skill(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skills::dsl::id;
	use crate::schema::skills::dsl::*;

	let deleted = diesel::delete(skills.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
