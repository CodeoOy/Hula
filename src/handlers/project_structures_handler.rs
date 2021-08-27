use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::projects::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::transactions::project_structure_transaction::*;
use crate::repositories::transactions::*;

#[derive(Deserialize, Debug)]
pub struct ProjectStructureData {
	pub name: String,
	pub description: Option<String>,
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

impl From<ProjectStructureData> for ProjectStructure {
	fn from(project: ProjectStructureData) -> ProjectStructure {
		ProjectStructure {
			name: project.name.clone(),
			description: project.description.clone(),
			is_hidden: project.is_hidden,
			needs: project
				.needs
				.iter()
				.map(|x| ProjectStructureNeed {
					label: x.label.clone(),
					count_of_users: x.count_of_users,
					begin_time: x.begin_time,
					end_time: x.end_time,
					percentage: x.percentage,
					skills: x
						.skills
						.iter()
						.map(|y| ProjectStructureNeedSkill {
							skill_label: y.skill_label.clone(),
							skillscopelevel_label: y.skillscopelevel_label.clone(),
							min_years: y.min_years,
							max_years: y.max_years,
							mandatory: y.mandatory,
						})
						.collect(),
				})
				.collect(),
		}
	}
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

	let projectdata: ProjectStructure = projectdata.into_inner().into();

	let res = web::block(move || {
		project_structure_transaction::save_project_structure(None, projectdata.into(), &pool, logged_user.email, false)
	})
	.await;
	match res {
		Ok(id) => Ok(HttpResponse::Ok().json(&id)),
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

	let projectdata: ProjectStructure = projectdata.into_inner().into();

	let res = web::block(move || {
		project_structure_transaction::save_project_structure(
			Some(id),
			projectdata.into(),
			&pool,
			logged_user.email,
			true,
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
