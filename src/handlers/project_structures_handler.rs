use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::projects::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct ProjectStructureData {
	pub name: String,
	pub is_hidden: bool,
	pub needs: Vec<ProjectStructureNeedData>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectStructureNeedData {
	pub label: String,
	pub count_of_users: i32,
	pub begin_time: chrono::NaiveDate,
	pub end_time: Option<chrono::NaiveDate>,
	pub percentage: Option<i32>,
	pub skills: Vec<ProjectStructureNeedSkillData>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectStructureNeedSkillData {
	pub skill_label: String,
	pub skillscopelevel_label: Option<String>,
	pub min_years: Option<f64>,
	pub max_years: Option<f64>,
	pub mandatory: bool,
}

pub async fn create_project_structure(
	projectdata: web::Json<ProjectStructureData>,
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

pub async fn update_project_structure(
	uuid_data: web::Path<String>,
	projectdata: web::Json<ProjectStructureData>,
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

	let res = web::block(move || {
		projects_repository::update_project(
			id,
			projectdata.name.clone(),
			projectdata.is_hidden,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
