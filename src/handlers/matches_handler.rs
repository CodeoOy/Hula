use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Serialize, Deserialize};

use crate::errors::ServiceError;
use crate::models::projectmatches::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	#[serde(default)] // default = 0
	pub ignore_mandatory: bool,
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
}

#[derive(Serialize, Debug)]
pub struct MatchDTO {
	pub user_id: uuid::Uuid,
	pub first_name: String,
	pub last_name: String,
	pub is_all_skills: bool,
	pub is_available: bool,
}

pub async fn get_all_matches(
	web::Query(q_query_data): web::Query<QueryData>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all matches: logged_user={:#?}", &_logged_user);

	let mut ignore_mandatory = false;
	if q_query_data.ignore_mandatory {
		trace!("IGNORING MANDATORY");
		ignore_mandatory = true;
	}

	let res = web::block(move || query_matches(ignore_mandatory, &pool)).await;

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

fn query_matches(
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
