use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::projects::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectData {
	pub name: String,
	pub available: bool,
}

#[derive(Deserialize, Debug)]
pub struct ProjectNeedData {
	pub project_id: uuid::Uuid,
	pub count_of_users: i32,
	pub begin_time: chrono::NaiveDate,
	pub end_time: Option<chrono::NaiveDate>,
	pub percentage: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectNeedSkillData {
	pub projectneed_id: uuid::Uuid,
	pub skill_id: uuid::Uuid,
	pub skillscopelevel_id: Option<uuid::Uuid>,
	pub min_years: Option<f64>,
	pub max_years: Option<f64>,
}

pub async fn get_all_projects(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all projects: logged_user={:#?}", &_logged_user);
	let res = web::block(move || projects_repository::query_all_projects(&pool)).await;

	match res {
		Ok(projects) => {
			if projects.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&projects));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_projectneedskills(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting project need skills: pid = {:#?} logged_user={:#?}",
		&pid,
		&_logged_user
	);
	let id = uuid::Uuid::parse_str(&pid.into_inner())?;

	let res = web::block(move || projectneedskills_repository::query_projectneedskills(id, &pool)).await;

	match res {
		Ok(projectneedskills) => {
			if projectneedskills.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&projectneedskills));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_project_needs(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting project needs: pid = {:#?} logged_user={:#?}",
		&pid,
		&_logged_user
	);
	let id = uuid::Uuid::parse_str(&pid.into_inner())?;

	let res = web::block(move || projectneeds_repository::query_project_needs(&pool, id)).await;

	match res {
		Ok(needs) => {
			if needs.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&needs));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_project(
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Create project: projectdata = {:#?} logged_user={:#?}",
		&projectdata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res =
		web::block(move || projects_repository::create_project(projectdata.name.clone(), logged_user.email, &pool))
			.await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_projectneed(
	projectneeddata: web::Json<ProjectNeedData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Create project need: projectneeddata = {:#?} logged_user={:#?}",
		&projectneeddata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		projectneeds_repository::create_projectneed(
			projectneeddata.project_id,
			projectneeddata.count_of_users,
			projectneeddata.percentage,
			projectneeddata.begin_time,
			projectneeddata.end_time,
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(projectneed) => Ok(HttpResponse::Ok().json(&projectneed)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_projectneedskill(
	projectneedskilldata: web::Json<ProjectNeedSkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Create project need skill: projectneedskilldata = {:#?} logged_user={:#?}",
		&projectneedskilldata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		projectneedskills_repository::create_projectneedskill(
			projectneedskilldata.projectneed_id,
			projectneedskilldata.skill_id,
			projectneedskilldata.skillscopelevel_id,
			projectneedskilldata.min_years,
			projectneedskilldata.max_years,
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(projectneedskill) => Ok(HttpResponse::Ok().json(&projectneedskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}


pub async fn update_projectneedskill(
	pid: web::Path<String>,
	projectneedskilldata: web::Json<ProjectNeedSkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Update project need skill: projectneedskilldata = {:#?} logged_user={:#?}",
		&projectneedskilldata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}
	let id = uuid::Uuid::parse_str(&pid.into_inner())?;
	
	let res = web::block(move || {
		projectneedskills_repository::update_projectneedskill(
			id,
			projectneedskilldata.projectneed_id,
			projectneedskilldata.skill_id,
			projectneedskilldata.skillscopelevel_id,
			projectneedskilldata.min_years,
			projectneedskilldata.max_years,
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(projectneedskill) => Ok(HttpResponse::Ok().json(&projectneedskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_pid(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting project by pid: pid={:#?}", &pid);

	let id = uuid::Uuid::parse_str(&pid.into_inner())?;

	let res = web::block(move || projects_repository::query_one(id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_project(
	uuid_data: web::Path<String>,
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Update project: uuid_data = {:#?} projectdata = {:#?} logged_user={:#?}",
		&uuid_data,
		&projectdata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res =
		web::block(move || projects_repository::update_project(id, projectdata.name.clone(), projectdata.available, logged_user.email, &pool))
			.await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_projectneed(
	uuid_data: web::Path<String>,
	projectneed: web::Json<ProjectNeedData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Update project need: uuid_data = {:#?} projectneed = {:#?} logged_user={:#?}",
		&uuid_data,
		&projectneed,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || {
		projectneeds_repository::update_projectneed(
			id,
			projectneed.count_of_users,
			projectneed.percentage,
			projectneed.begin_time,
			projectneed.end_time,
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(need) => Ok(HttpResponse::Ok().json(&need)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_project(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete project: uuid_data = {:#?} logged_user={:#?}",
		&uuid_data,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || projects_repository::delete_project(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_projectneed(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting project need: uuid_data = {:#?} logged_user={:#?}",
		&uuid_data,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || projectneeds_repository::delete_projectneed(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_projectneedskill(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete project need skill: uuid_data = {:#?} logged_user={:#?}",
		&uuid_data,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || projectneedskills_repository::delete_projectneedskill(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
