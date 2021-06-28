use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::projects::{Pool, Project};

pub fn query_all_projects(pool: web::Data<Pool>) -> Result<Vec<Project>, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projects.load::<Project>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn query_create_project(
	project_name: String,
	pool: web::Data<Pool>,
	email: String,
) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_project = Project {
		id: uuid::Uuid::new_v4(),
		available: true,
		name: project_name,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(projects)
		.values(&new_project)
		.get_result::<Project>(conn);
	if rows_inserted.is_ok() {
		return Ok(new_project.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_one(pid: String, pool: web::Data<Pool>) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::{id, projects};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&pid)?;
	let mut items = projects.filter(id.eq(uuid_query)).load::<Project>(conn)?;
	if let Some(project_res) = items.pop() {
		return Ok(project_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_update_project(
	uuid_data: String,
	project_name: String,
	pool: web::Data<Pool>,
	email: String,
) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::id;
	use crate::schema::projects::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(projects)
		.filter(id.eq(uuid_query))
		.set((name.eq(project_name), updated_by.eq(email.clone())))
		.load::<Project>(conn)?;
	if let Some(_project_res) = items.pop() {
		return Ok(());
	}

	Ok(())
}

pub fn query_delete_project(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::id;
	use crate::schema::projects::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(projects.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(()) // TODO: Error handling. This query returns ok even if the delete doesn't happen
}

