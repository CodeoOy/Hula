use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;
use chrono::NaiveDateTime;

use crate::models::projects::{Pool, ProjectNeed};

pub fn query_project_needs(
	pool: &web::Data<Pool>,
	pid: uuid::Uuid,
) -> Result<Vec<ProjectNeed>, Error> {
	use crate::schema::projectneeds::dsl::{project_id, projectneeds};
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projectneeds.filter(project_id.eq(pid)).load::<ProjectNeed>(conn)?;
	Ok(items.into())
}

pub fn create_projectneed(
	q_project_id: uuid::Uuid,
	q_count_of_users: i32,
	q_percentage: Option<i32>,
	q_begin_time: NaiveDateTime,
	q_end_time: Option<NaiveDateTime>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<ProjectNeed, Error> {
	use crate::schema::projectneeds::dsl::projectneeds;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_projectneed = ProjectNeed {
		id: uuid::Uuid::new_v4(),
		project_id: q_project_id,
		count_of_users: q_count_of_users,
		percentage: q_percentage,
		begin_time: q_begin_time,
		end_time: q_end_time,
		updated_by: q_email,
	};
	
	diesel::insert_into(projectneeds)
		.values(&new_projectneed)
		.execute(conn)?;

	Ok(new_projectneed.into())
}

pub fn update_projectneed(
	uuid_data: uuid::Uuid,
	q_count_of_users: i32,
	q_percentage: Option<i32>,
	q_begin_time: NaiveDateTime,
	q_end_time: Option<NaiveDateTime>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Option<ProjectNeed>, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::{projectneeds, *};

	let mut items = diesel::update(projectneeds)
		.filter(id.eq(uuid_data))
		.set((
			count_of_users.eq(q_count_of_users),
			percentage.eq(q_percentage),
			begin_time.eq(q_begin_time),
			end_time.eq(q_end_time),
			updated_by.eq(q_email),
		))
		.load::<ProjectNeed>(conn)?;

	if let Some(need_res) = items.pop() {
		return Ok(Some(need_res));
	}

	Ok(None)
}

pub fn delete_projectneed(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<usize, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::id;
	use crate::schema::projectneeds::dsl::*;

	let deleted = diesel::delete(projectneeds.filter(id.eq(uuid_data))).execute(conn)?;
	Ok(deleted)
}

