use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::projects::{Pool, ProjectNeedSkill};

pub fn query_projectneedskills(
	pid: String,
	pool: web::Data<Pool>,
) -> Result<Vec<ProjectNeedSkill>, crate::errors::ServiceError> {
	use crate::schema::projectneedskills::dsl::{projectneed_id, projectneedskills};
	let conn: &PgConnection = &pool.get().unwrap();
	let pid = uuid::Uuid::parse_str(&pid)?;
	let items = projectneedskills
		.filter(projectneed_id.eq(pid))
		.load::<ProjectNeedSkill>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}
pub fn query_create_projectneedskill(
	projectneedskilldata: web::Json<ProjectNeedSkill>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<ProjectNeedSkill, crate::errors::ServiceError> {
	use crate::schema::projectneedskills::dsl::projectneedskills;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_projectneedskill = ProjectNeedSkill {
		id: uuid::Uuid::new_v4(),
		projectneed_id: projectneedskilldata.projectneed_id,
		skill_id: projectneedskilldata.skill_id,
		skillscopelevel_id: projectneedskilldata.skillscopelevel_id,
		min_years: projectneedskilldata.min_years,
		max_years: projectneedskilldata.max_years,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(projectneedskills)
		.values(&new_projectneedskill)
		.get_result::<ProjectNeedSkill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		return Ok(new_projectneedskill.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_delete_projectneedskill(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneedskills::dsl::id;
	use crate::schema::projectneedskills::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(projectneedskills.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
