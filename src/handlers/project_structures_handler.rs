use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::projects::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::{
	projectmatches_repository, projectneeds_repository, projectneedskills_repository, projects_repository,
	projectskills_repository, skills_repository, skillscopelevels_repository,
};

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

#[derive(Serialize, Debug)]
pub struct ProjectStructureResponse {
	pub id: uuid::Uuid,
	pub matches: i32,
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
		web::block(move || save_project_structure(None, projectdata.into_inner(), &pool, logged_user.email, false))
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

	let res =
		web::block(move || save_project_structure(Some(id), projectdata.into_inner(), &pool, logged_user.email, true))
			.await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub fn save_project_structure(
	mut id: Option<uuid::Uuid>,
	project: ProjectStructureData,
	pool: &web::Data<Pool>,
	email: String,
	is_update: bool,
) -> Result<ProjectStructureResponse, ServiceError> {
	if is_update == true {
		trace!("Checking if project structure needs update. id={}", &id.unwrap());
		let equals = test_project_structure_equals(id.unwrap(), &project, &pool)?;

		if equals == true {
			trace!("No update needed. id={}", &id.unwrap());
			return Ok(ProjectStructureResponse {
				id: id.unwrap(),
				matches: get_project_matches_count(id.unwrap(), &pool)?,
			});
		}
		trace!("Update is needed. id={}", &id.unwrap());
	} else {
		trace!("Creating a project. name={}", &project.name.clone());
		let db_project = projects_repository::create_project(
			project.name.clone(),
			project.description.clone(),
			project.is_hidden,
			email.clone(),
			&pool,
		)?;
		id = Some(db_project.id);
		trace!("id={}", &id.unwrap());
	}

	let id = id.unwrap();

	trace!("Updating project. id={}", &id);
	projects_repository::update_project(
		id,
		project.name.clone(),
		project.description.clone(),
		project.is_hidden,
		email.clone(),
		&pool,
	)?;

	trace!("Deleting all project needs. id={}", &id);
	projectneeds_repository::delete_projectneeds_by_project(id, &pool)?;

	for need in project.needs.iter() {
		trace!("Creating a project need. label={}", &need.label.clone());
		let need_res = projectneeds_repository::create_projectneed(
			id,
			need.count_of_users,
			Some(need.label.clone()),
			need.percentage,
			need.begin_time,
			need.end_time,
			email.clone(),
			&pool,
		)?;

		for skill in need.skills.iter() {
			trace!("Searching a skill. label={}", &skill.skill_label.clone());
			let db_skill = skills_repository::get_skill_by_label(skill.skill_label.clone(), &pool);
			if db_skill.is_err() {
				trace!("Skill not found.");
				continue;
			}

			let db_skill = db_skill.unwrap();

			let mut skill_scope_level_id: Option<uuid::Uuid> = None;
			if let Some(scopelabel) = skill.skillscopelevel_label.clone() {
				trace!("Searching a skill level. label={}", scopelabel);

				let db_skill_scope_level = skillscopelevels_repository::get_skill_level_by_label(scopelabel, &pool);

				if db_skill_scope_level.is_err() == false {
					let db_skill_scope_level = db_skill_scope_level.unwrap();
					skill_scope_level_id = Some(db_skill_scope_level.id);
					trace!("Found. id={}", skill_scope_level_id.unwrap());
				} else {
					trace!("Skill level not found.");
				}
			}

			trace!(
				"Creating a project need skill. need_id={}, skill_id={}",
				&need_res.id,
				&db_skill.id
			);
			projectneedskills_repository::create_projectneedskill(
				need_res.id,
				db_skill.id,
				skill_scope_level_id,
				skill.min_years,
				skill.max_years,
				email.clone(),
				skill.mandatory,
				&pool,
			)?;
		}
	}

	Ok(ProjectStructureResponse {
		id: id,
		matches: get_project_matches_count(id, &pool)?,
	})
}

fn get_project_matches_count(id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<i32, ServiceError> {
	let db_project = projects_repository::query_one(id, &pool)?;
	let db_matches = projectmatches_repository::find_by_project(&db_project, &pool)?;

	Ok(db_matches.len() as i32)
}

fn test_project_structure_equals(
	id: uuid::Uuid,
	project: &ProjectStructureData,
	pool: &web::Data<Pool>,
) -> Result<bool, ServiceError> {
	let db_project = projects_repository::query_one(id, &pool)?;

	if db_project.name != project.name || db_project.is_hidden != project.is_hidden {
		trace!("Project fields differ.");
		return Ok(false);
	}

	let db_project_skills = projectskills_repository::find_by_projects(&vec![db_project], &pool)?;
	let db_project_skills = &db_project_skills[0];

	let mut skills_count = 0;

	for need in project.needs.iter() {
		skills_count += need.skills.len();

		for skill in need.skills.iter() {
			let db_project_skill = db_project_skills
				.iter()
				.filter(|x| x.pn_label.clone().unwrap_or_default() == need.label && x.skill_label == skill.skill_label)
				.next();

			if db_project_skill.is_none() {
				trace!("Skill missing in db.");
				return Ok(false);
			}

			let db_project_skill = db_project_skill.unwrap();

			if db_project_skill.is_mandatory != skill.mandatory
				|| db_project_skill.required_minyears != skill.min_years
				|| db_project_skill.required_label != skill.skillscopelevel_label
			{
				trace!("Difference in skill.");
				return Ok(false);
			}
		}
	}

	if skills_count != db_project_skills.len() {
		trace!("Different number of skills.");
		return Ok(false);
	}

	Ok(true)
}
