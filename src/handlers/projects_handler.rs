use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::projects::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	#[serde(default)] // default = 0
	pub is_include_skills_and_matches: bool,
}

#[derive(Deserialize, Debug)]
pub struct ProjectData {
	pub name: String,
	pub is_hidden: bool,
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
	pub mandatory: bool,
}

#[derive(Serialize, Debug)]
pub struct ProjectDTO {
	pub id: uuid::Uuid,
	pub name: String,
	pub is_hidden: bool,
	pub skills: Vec<SkillDTO>,
	pub matches: Vec<MatchDTO>,
}

#[derive(Serialize, Debug)]
pub struct SkillDTO {
	pub skill_label: String,
	pub skill_mandatory: bool,
}

#[derive(Serialize, Debug)]
pub struct MatchDTO {
	pub user_id: uuid::Uuid,
	pub first_name: String,
	pub last_name: String,
	pub is_all_skills: bool,
	pub is_available: bool,
}

pub async fn get_all_projects(
	web::Query(q_query_data): web::Query<QueryData>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all projects: logged_user={:#?}", &_logged_user);

	let mut is_include = false;

	//if let Some(query_data) = q_query_data {
	if q_query_data.is_include_skills_and_matches {
		trace!("IS INCLUDE");
		is_include = true;
	}
	//}

	let res = web::block(move || query_projects_dto(is_include, &pool)).await;

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
			"label".to_string(),
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
			projectneedskilldata.mandatory,
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
			projectneedskilldata.mandatory,
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

fn query_projects_dto(
	include_matches_and_skills: bool,
	pool: &web::Data<Pool>,
) -> Result<Vec<ProjectDTO>, ServiceError> {
	use crate::models::projectmatches::ProjectMatch;
	use crate::models::projectskills::ProjectSkill;

	let projects = projects_repository::query_all_projects(&pool)?;
	let mut skills: Vec<Vec<ProjectSkill>> = vec![];
	let mut matches: Vec<Vec<ProjectMatch>> = vec![];

	if include_matches_and_skills {
		skills = projectskills_repository::find_by_projects(&projects, &pool)?;
		matches = projectmatches_repository::find_by_projects(&projects, &pool)?;
	}

	let mut dtos: Vec<ProjectDTO> = vec![];

	for idx in 0..projects.len() {
		let project = &projects[idx];

		let mut skills_vec: Vec<SkillDTO> = vec![];
		let mut matches_vec: Vec<MatchDTO> = vec![];

		if include_matches_and_skills {
			for s in &skills[idx] {
				let ss = SkillDTO {
					skill_label: s.skill_label.clone(),
					skill_mandatory: s.is_mandatory,
				};
				skills_vec.push(ss);
			}

			let matches = &matches[idx];

			for s in matches {
				if matches_vec.iter().any(|x| x.user_id == s.user_id) {
					continue;
				}

				let user_matches = matches.iter().filter(|x| x.user_id == s.user_id);
				let is_all_skills = skills_vec
					.iter()
					.all(|x| user_matches.clone().any(|y| x.skill_label == y.skill_label));
				let is_user_available = user_matches
					.clone()
					.any(|x| x.required_load.unwrap_or_default() >= x.user_load);

				let ss2 = MatchDTO {
					user_id: s.user_id.clone(),
					first_name: s.user_first_name.clone(),
					last_name: s.user_last_name.clone(),
					is_all_skills: is_all_skills,
					is_available: is_user_available,
				};
				matches_vec.push(ss2);
			}

			// Sort matches
			matches_vec.sort_by(|a, b| b.is_all_skills.cmp(&a.is_all_skills));
			matches_vec.sort_by(|a, b| b.is_available.cmp(&a.is_available));
		}

		let project_dto = ProjectDTO {
			id: project.id.clone(),
			name: project.name.clone(),
			is_hidden: project.is_hidden,
			skills: skills_vec,
			matches: matches_vec,
		};
		dtos.push(project_dto);
	}

	Ok(dtos)
}
