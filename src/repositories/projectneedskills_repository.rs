use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;
use diesel::result::Error::NotFound;

use crate::models::projects::{Pool, ProjectNeedSkill};

pub fn query_projectneedskills(
	pid: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<ProjectNeedSkill>, Error> {
	use crate::schema::projectneedskills::dsl::{projectneed_id, projectneedskills, skill_id};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = projectneedskills
		.filter(projectneed_id.eq(pid))
		.order(skill_id.asc())
		.load::<ProjectNeedSkill>(conn)?;

	Ok(items)
}

pub fn create_projectneedskill(
	q_projectneed_id: uuid::Uuid,
	q_skill_id: uuid::Uuid,
	q_skillscopelevel_id: Option<uuid::Uuid>,
	q_min_years: Option<f64>,
	q_max_years: Option<f64>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<ProjectNeedSkill, Error> {
	use crate::schema::projectneedskills::dsl::projectneedskills;
	let conn: &PgConnection = &pool.get().unwrap();
	
	let new_projectneedskill = ProjectNeedSkill {
		id: uuid::Uuid::new_v4(),
		projectneed_id: q_projectneed_id,
		skill_id: q_skill_id,
		skillscopelevel_id: q_skillscopelevel_id,
		min_years: q_min_years,
		max_years: q_max_years,
		updated_by: q_email,
	};

	let projectneedskill = diesel::insert_into(projectneedskills)
		.values(&new_projectneedskill)
		.get_result::<ProjectNeedSkill>(conn)?;

	Ok(projectneedskill)
}

pub fn delete_projectneedskill(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneedskills::dsl::id;
	use crate::schema::projectneedskills::dsl::*;

	let deleted = diesel::delete(projectneedskills.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
