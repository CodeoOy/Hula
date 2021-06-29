use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::projects::{Pool, ProjectNeed};

pub fn query_project_needs(
	pool: web::Data<Pool>,
	pid_path: String,
) -> Result<Vec<ProjectNeed>, crate::errors::ServiceError> {
	use crate::schema::projectneeds::dsl::{project_id, projectneeds};
	let conn: &PgConnection = &pool.get().unwrap();
	let pid = uuid::Uuid::parse_str(&pid_path)?;
	let items = projectneeds.filter(project_id.eq(pid)).load::<ProjectNeed>(conn)?;
	if items.is_empty() == false {
		return Ok(items.into());
	}
	Err(ServiceError::Empty)
}

pub fn query_create_projectneed(
	projectneeddata: web::Json<ProjectNeed>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<ProjectNeed, crate::errors::ServiceError> {
	use crate::schema::projectneeds::dsl::projectneeds;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_projectneed = ProjectNeed {
		id: uuid::Uuid::new_v4(),
		project_id: projectneeddata.project_id,
		count_of_users: projectneeddata.count_of_users,
		percentage: projectneeddata.percentage,
		begin_time: projectneeddata.begin_time,
		end_time: projectneeddata.end_time,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(projectneeds)
		.values(&new_projectneed)
		.get_result::<ProjectNeed>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		return Ok(new_projectneed.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_update_projectneed(
	uuid_data: String,
	projectneed: web::Json<ProjectNeed>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::{projectneeds, *};

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(projectneeds)
		.filter(id.eq(uuid_query))
		.set((
			project_id.eq(projectneed.project_id),
			count_of_users.eq(projectneed.count_of_users),
			percentage.eq(projectneed.percentage),
			begin_time.eq(projectneed.begin_time),
			end_time.eq(projectneed.end_time),
			updated_by.eq(email.clone()),
		))
		.load::<ProjectNeed>(conn)?;
	if let Some(_project_res) = items.pop() {
		return Ok(());
	}
	Ok(())
}

pub fn query_delete_projectneed(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::id;
	use crate::schema::projectneeds::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(projectneeds.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}

