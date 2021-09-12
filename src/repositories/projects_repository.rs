use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

use crate::models::projects::{Pool, Project};

pub fn query_all_projects(pool: &web::Data<Pool>) -> Result<Vec<Project>, Error> {
	use crate::schema::projects::dsl::{name, projects};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = projects.order(name.asc()).load::<Project>(conn)?;

	Ok(items)
}

pub fn query_one(uuid_query: uuid::Uuid, is_admin: bool, pool: &web::Data<Pool>) -> Result<Project, Error> {
	use crate::schema::projects::dsl::{id, projects};
	let conn: &PgConnection = &pool.get().unwrap();

	let project = projects.filter(id.eq(uuid_query)).get_result::<Project>(conn)?;

	if project.is_hidden == true && is_admin == false {
		return Err(NotFound);
	}

	Ok(project)
}

pub fn create_project(
	q_project_name: String,
	q_project_description: Option<String>,
	q_is_hidden: bool,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Project, Error> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_project = Project {
		id: uuid::Uuid::new_v4(),
		description: q_project_description,
		inserted_at: chrono::Local::now().naive_local(),
		is_hidden: q_is_hidden,
		name: q_project_name,
		updated_by: q_email,
	};

	let project = diesel::insert_into(projects)
		.values(&new_project)
		.get_result::<Project>(conn)?;

	Ok(project)
}

pub fn update_project(
	uuid_data: uuid::Uuid,
	q_project_name: String,
	q_project_description: Option<String>,
	q_project_is_hidden: bool,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Project, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::id;
	use crate::schema::projects::dsl::*;

	let project = diesel::update(projects)
		.filter(id.eq(uuid_data))
		.set((
			name.eq(q_project_name),
			description.eq(q_project_description),
			is_hidden.eq(q_project_is_hidden),
			updated_by.eq(q_email.clone()),
		))
		.get_result::<Project>(conn)?;

	Ok(project)
}

pub fn delete_project(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::id;
	use crate::schema::projects::dsl::*;

	let deleted = diesel::delete(projects.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
