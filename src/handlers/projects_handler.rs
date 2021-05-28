use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::projects::{Pool, Project, ProjectSkill};
use crate::models::users::LoggedUser;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectData {
	pub name: String,
}

pub async fn get_all_projects(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	println!("\nGetting all projects");
	let res = web::block(move || query_all_projects(pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_all_projects(pool: web::Data<Pool>) -> Result<Vec<Project>, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projects.load::<Project>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all projects.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn get_project_skills(
	pool: web::Data<Pool>,
	pid: web::Path<String>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("\nGetting all projects");
	let res = web::block(move || query_project_skills(pool, pid.into_inner())).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_project_skills(
	pool: web::Data<Pool>,
	pid_path: String,
) -> Result<Vec<ProjectSkill>, crate::errors::ServiceError> {
	use crate::schema::projectskills::dsl::{project_id, projectskills};
	let conn: &PgConnection = &pool.get().unwrap();
	let pid = uuid::Uuid::parse_str(&pid_path)?;
	let items = projectskills.filter(project_id.eq(pid)).load::<ProjectSkill>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all projects.\n");
		return Ok(items.into());
	}
	Err(ServiceError::Empty)
}

pub async fn create_project(
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a project");
	let res = web::block(move || query_create_project(projectdata, pool, logged_user.email)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_project(
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_project = Project {
		id: uuid::Uuid::new_v4(),
		available: true,
		name: projectdata.name.clone(),
		updated_by: email,
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(projects)
		.values(&new_project)
		.get_result::<Project>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nProject added successfully.\n");
		return Ok(new_project.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn create_projectskill(
	projectskilldata: web::Json<ProjectSkill>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a project skill");
	let res = web::block(move || query_create_projectskill(projectskilldata, pool, logged_user.email)).await;
	match res {
		Ok(projectskill) => Ok(HttpResponse::Ok().json(&projectskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_projectskill(
	projectskilldata: web::Json<ProjectSkill>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<ProjectSkill, crate::errors::ServiceError> {
	use crate::schema::projectskills::dsl::projectskills;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_projectskill = ProjectSkill {
		id: uuid::Uuid::new_v4(),
		project_id: projectskilldata.project_id,
		skill_id: projectskilldata.skill_id,
		skillscopelevel_id: projectskilldata.skillscopelevel_id,
		min_years: projectskilldata.min_years,
		max_years: projectskilldata.max_years,
		countofusers: projectskilldata.countofusers,
		begin_time: chrono::Local::now().naive_local(),
		end_time: chrono::Local::now().naive_local() + chrono::Duration::hours(240),
		percentage: projectskilldata.percentage,
		updated_by: email,
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(projectskills)
		.values(&new_projectskill)
		.get_result::<ProjectSkill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nProject skill added successfully.\n");
		return Ok(new_projectskill.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn get_by_pid(
	uuid_data: web::Json<QueryData>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("\nGetting project by uuid");
	let res = web::block(move || query_one(uuid_data.into_inner(), pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_one(uuid_data: QueryData, pool: web::Data<Pool>) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::{id, projects};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data.id)?;
	let mut items = projects.filter(id.eq(uuid_query)).load::<Project>(conn)?;
	if let Some(project_res) = items.pop() {
		println!("\nQuery successful.\n");
		return Ok(project_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn delete_project(uuid_data: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_delete_project(uuid_data.into_inner(), pool)).await;
	println!("\nProject deleted\n");
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_project(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::id;
	use crate::schema::projects::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(projects.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}

pub async fn delete_all_projects(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_delete_all_projects(pool)).await;
	println!("\nAll projects deleted\n");
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_all_projects(pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::*;

	diesel::delete(projects).execute(conn)?;
	Ok(())
}
