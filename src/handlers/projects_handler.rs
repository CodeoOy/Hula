use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::projects::{Pool, Project, ProjectNeed, ProjectNeedSkill};
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

pub async fn get_projectneedskills(
	pool: web::Data<Pool>, 
	pid: web::Path<String>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("\nGetting all project need skills");
	let res = web::block(move || query_projectneedskills(pool, pid.into_inner())).await;

	match res {
		Ok(projectneedskill) => Ok(HttpResponse::Ok().json(&projectneedskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_projectneedskills(
	pool: web::Data<Pool>,
	pid: String
) -> Result<Vec<ProjectNeedSkill>, crate::errors::ServiceError> {
	use crate::schema::projectneedskills::dsl::projectneedskills;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projectneedskills.load::<ProjectNeedSkill>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all project need skills.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn get_project_needs(
	pool: web::Data<Pool>,
	pid: web::Path<String>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("\nGetting all projects");
	let res = web::block(move || query_project_needs(pool, pid.into_inner())).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_project_needs(
	pool: web::Data<Pool>,
	pid_path: String,
) -> Result<Vec<ProjectNeed>, crate::errors::ServiceError> {
	use crate::schema::projectneeds::dsl::{project_id, projectneeds};
	let conn: &PgConnection = &pool.get().unwrap();
	let pid = uuid::Uuid::parse_str(&pid_path)?;
	let items = projectneeds.filter(project_id.eq(pid)).load::<ProjectNeed>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all project needs.\n");
		return Ok(items.into());
	}
	println!("\nNo needs in project.\n");
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

pub async fn create_projectneed(
	projectneeddata: web::Json<ProjectNeed>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a project skill");
	let res = web::block(move || query_create_projectneed(projectneeddata, pool, logged_user.email)).await;
	match res {
		Ok(projectneed) => Ok(HttpResponse::Ok().json(&projectneed)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_projectneed(
	projectneeddata: web::Json<ProjectNeed>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<ProjectNeed, crate::errors::ServiceError> {
	use crate::schema::projectneeds::dsl::projectneeds;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_projectneed = ProjectNeed {
		id: uuid::Uuid::new_v4(),
		project_id: projectneeddata.project_id,
		count_of_users: projectneeddata.count_of_users,
		percentage: projectneeddata.percentage,
		begin_time: projectneeddata.begin_time,
		end_time: projectneeddata.end_time,
		updated_by: email,
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(projectneeds)
		.values(&new_projectneed)
		.get_result::<ProjectNeed>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nProject need added successfully.\n");
		return Ok(new_projectneed.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn create_projectneedskill(
	projectneedskilldata: web::Json<ProjectNeedSkill>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a project skill");
	let res = web::block(move || query_create_projectneedskill(projectneedskilldata, pool, logged_user.email)).await;
	match res {
		Ok(projectneedskill) => Ok(HttpResponse::Ok().json(&projectneedskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_projectneedskill(
	projectneedskilldata: web::Json<ProjectNeedSkill>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<ProjectNeedSkill, crate::errors::ServiceError> {
	use crate::schema::projectneedskills::dsl::projectneedskills;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_projectneedskill = ProjectNeedSkill {
		id: uuid::Uuid::new_v4(),
		projectneed_id: projectneedskilldata.projectneed_id,
		skill_id: projectneedskilldata.skill_id,
		skillscopelevel_id: projectneedskilldata.skillscopelevel_id,
		min_years: projectneedskilldata.min_years,
		max_years: projectneedskilldata.max_years,
		updated_by: email,
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(projectneedskills)
		.values(&new_projectneedskill)
		.get_result::<ProjectNeedSkill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nProject need added successfully.\n");
		return Ok(new_projectneedskill.into());
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
