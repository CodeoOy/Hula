use actix_web::web;
use chrono::NaiveDate;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

use crate::models::projects::{Pool, ProjectNeed};

pub fn query_project_needs(pool: &web::Data<Pool>, pid: uuid::Uuid) -> Result<Vec<ProjectNeed>, Error> {
	use crate::schema::projectneeds::dsl::{begin_time, project_id, projectneeds};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = projectneeds
		.filter(project_id.eq(pid))
		.order(begin_time.asc())
		.load::<ProjectNeed>(conn)?;

	Ok(items)
}

pub fn create_projectneed(
	q_project_id: uuid::Uuid,
	q_label: String,
	q_count_of_users: i32,
	q_percentage: Option<i32>,
	q_begin_time: NaiveDate,
	q_end_time: Option<NaiveDate>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<ProjectNeed, Error> {
	use crate::schema::projectneeds::dsl::projectneeds;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_projectneed = ProjectNeed {
		id: uuid::Uuid::new_v4(),
		label: q_label.clone(),
		project_id: q_project_id,
		count_of_users: q_count_of_users,
		percentage: q_percentage,
		begin_time: q_begin_time,
		end_time: q_end_time,
		updated_by: q_email,
	};

	let projectneed = diesel::insert_into(projectneeds)
		.values(&new_projectneed)
		.get_result::<ProjectNeed>(conn)?;

	Ok(projectneed)
}

pub fn update_projectneed(
	uuid_data: uuid::Uuid,
	q_count_of_users: i32,
	q_percentage: Option<i32>,
	q_begin_time: NaiveDate,
	q_end_time: Option<NaiveDate>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<ProjectNeed, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::{projectneeds, *};

	let projectneed = diesel::update(projectneeds)
		.filter(id.eq(uuid_data))
		.set((
			count_of_users.eq(q_count_of_users),
			percentage.eq(q_percentage),
			begin_time.eq(q_begin_time),
			end_time.eq(q_end_time),
			updated_by.eq(q_email),
		))
		.get_result::<ProjectNeed>(conn)?;

	Ok(projectneed)
}

pub fn delete_projectneed(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::id;
	use crate::schema::projectneeds::dsl::*;

	let deleted = diesel::delete(projectneeds.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}

pub fn delete_projectneeds_by_project(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::project_id;
	use crate::schema::projectneeds::dsl::*;

	diesel::delete(projectneeds.filter(project_id.eq(uuid_data))).execute(conn)?;
 	Ok(())
}
