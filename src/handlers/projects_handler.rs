use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;
use log::trace;

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
	trace!("Getting all projects: logged_user={:#?}", &_logged_user);
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
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn get_projectneedskills(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting project need skills: pid = {:#?} logged_user={:#?}", &pid, &_logged_user);
	let res = web::block(move || query_projectneedskills(pid.into_inner(), pool)).await;

	match res {
		Ok(projectneedskill) => Ok(HttpResponse::Ok().json(&projectneedskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_projectneedskills(
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

pub async fn get_project_needs(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting project needs: pid = {:#?} logged_user={:#?}", &pid, &_logged_user);
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
		return Ok(items.into());
	}
	Err(ServiceError::Empty)
}

pub async fn create_project(
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Create project: projectdata = {:#?} logged_user={:#?}", &projectdata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

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
	let new_project = Project {
		id: uuid::Uuid::new_v4(),
		available: true,
		name: projectdata.name.clone(),
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

pub async fn create_projectneed(
	projectneeddata: web::Json<ProjectNeed>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Create project need: projectneeddata = {:#?} logged_user={:#?}", &projectneeddata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

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

pub async fn create_projectneedskill(
	projectneedskilldata: web::Json<ProjectNeedSkill>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Create project need skill: projectneedskilldata = {:#?} logged_user={:#?}", &projectneedskilldata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

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

pub async fn get_by_pid(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting project by pid: pid={:#?}", &pid);
	let res = web::block(move || query_one(pid.into_inner(), pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_one(pid: String, pool: web::Data<Pool>) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::{id, projects};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&pid)?;
	let mut items = projects.filter(id.eq(uuid_query)).load::<Project>(conn)?;
	if let Some(project_res) = items.pop() {
		return Ok(project_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn update_project(
	uuid_data: web::Path<String>,
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Update project: uuid_data = {:#?} projectdata = {:#?} logged_user={:#?}", &uuid_data, &projectdata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

	let res =
		web::block(move || query_update_project(uuid_data.into_inner(), projectdata, pool, logged_user.email)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_update_project(
	uuid_data: String,
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projects::dsl::id;
	use crate::schema::projects::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(projects)
		.filter(id.eq(uuid_query))
		.set((name.eq(projectdata.name.clone()), updated_by.eq(email.clone())))
		.load::<Project>(conn)?;
	if let Some(_project_res) = items.pop() {
		return Ok(());
	}

	Ok(())
}

pub async fn update_projectneed(
	uuid_data: web::Path<String>,
	projectneed: web::Json<ProjectNeed>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Update project need: uuid_data = {:#?} projectneed = {:#?} logged_user={:#?}", &uuid_data, &projectneed, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

	let res =
		web::block(move || query_update_projectneed(uuid_data.into_inner(), projectneed, pool, logged_user.email))
			.await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_update_projectneed(
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

pub async fn delete_project(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Delete project: uuid_data = {:#?} logged_user={:#?}", &uuid_data, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

	let res = web::block(move || query_delete_project(uuid_data.into_inner(), pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
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
	Ok(()) // TODO: Error handling. This query returns ok even if the delete doesn't happen
}

pub async fn delete_projectneed(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Deleting project need: uuid_data = {:#?} logged_user={:#?}", &uuid_data, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

	let res = web::block(move || query_delete_projectneed(uuid_data.into_inner(), pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_projectneed(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneeds::dsl::id;
	use crate::schema::projectneeds::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(projectneeds.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}

pub async fn delete_projectneedskill(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Delete project need skill: uuid_data = {:#?} logged_user={:#?}", &uuid_data, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Unauthorized);
	}

	let res = web::block(move || query_delete_projectneedskill(uuid_data.into_inner(), pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_projectneedskill(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::projectneedskills::dsl::id;
	use crate::schema::projectneedskills::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(projectneedskills.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
