use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Serialize, Deserialize};

use crate::errors::ServiceError;
use crate::models::projectmatches::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

const TIER1: i32 = 1;
const TIER2: i32 = 2;
const TIER3: i32 = 3;
const TIER4: i32 = 4;
const TIER5: i32 = 5;
const TIER6: i32 = 6;

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
	pub skill_mandatory: bool,
}

#[derive(Serialize, Debug)]
pub struct MatchDTO {
	pub user_id: uuid::Uuid,
	pub first_name: String,
	pub last_name: String,
	pub has_mandatory: bool,
	pub is_all_skills: bool,
	pub is_available: bool,
	pub tier: i32,
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
	ignore_mandatory: bool,
	pool: &web::Data<Pool>,
) -> Result<Vec<ProjectDTO>, ServiceError> {
	use crate::models::projectmatches::ProjectMatch;
	use crate::models::projectskills::ProjectSkill;

	let projects = projects_repository::query_all_projects(&pool)?;
	let mut skills: Vec<Vec<ProjectSkill>> = vec![];
	let mut matches: Vec<Vec<ProjectMatch>> = vec![];

	skills = projectskills_repository::find_by_projects(&projects, &pool)?;
	matches = projectmatches_repository::find_by_projects(&projects, &pool)?;

	let mut dtos: Vec<ProjectDTO> = vec![];

	for idx in 0..projects.len() {
		let project = &projects[idx];

		let mut skills_vec: Vec<SkillDTO> = vec![];
		let mut matches_vec: Vec<MatchDTO> = vec![];

		for s in &skills[idx] {
			let ss = SkillDTO {
				skill_label: s.skill_label.clone(),
				skill_mandatory: s.is_mandatory,
			};
			skills_vec.push(ss);
		}

		let matches = &matches[idx];
		
		/*
		SKILL: ProjectMatch { idx: 8, project_id: c1b7e37e-b022-4712-8a1b-d73030ecd7cc, 
		skill_label: "Rust", skill_mandatory: false, pn_id: ef081bab-3591-47dd-8187-9cf66a977386, 
		pn_label: "", required_load: Some(90), required_index: Some(3), required_minyears: Some(2.0), 
		required_maxyears: None, user_id: 72e3c6f6-4ba4-43a2-a852-ef74dc9f9373, user_first_name: "Tuomas", 
		user_last_name: "Codeooo", user_is_hidden: false, user_load: 0, user_index: Some(1), user_years: Some(4.0) }
		*/

		for s in matches {
			if matches_vec.iter().any(|x| x.user_id == s.user_id) {
				continue;
			}
			let user_matches = matches.iter().filter(|x| x.user_id == s.user_id);
			let is_all_skills = skills_vec
				.iter()
				.all(|x| user_matches.clone().any(|y| {
					x.skill_label == y.skill_label
				}));
			let is_user_available = user_matches
				.clone()
				.any(|x| x.required_load.unwrap_or_default() >= x.user_load);
			// let has_mandatory_skill = skills_vec
			// 	.iter()
			// 	.all(|_x| user_matches.clone().any(|y| y.skill_mandatory == true));
			let mandatory_skills = skills_vec
				.iter()
				.filter(|x| x.skill_mandatory == true);
			let has_mandatory_skills = mandatory_skills
				.clone()
				.all(|x| user_matches.clone().any(|y| {
					println!("\nSkill: {:?} \t\tM: {:?}\n", y.skill_label, y.skill_mandatory); 
					x.skill_label == y.skill_label
				}));
			println!("MATCH MANDATORY: {:?}\n\n", has_mandatory_skills);

			let tier: i32 = match (has_mandatory_skills, is_user_available, is_all_skills) {
				(true, true, true) => TIER1,
				(true, true, false) => TIER1,
				(true, false, true) => TIER2,
				(false, true, true) => TIER3,
				(false, true, false) => TIER4,
				(false, false, true) => TIER5,
				_ => TIER6,
			};

			let ss2 = MatchDTO {
				user_id: s.user_id.clone(),
				first_name: s.user_first_name.clone(),
				last_name: s.user_last_name.clone(),
				has_mandatory: has_mandatory_skills,
				is_all_skills: is_all_skills,
				is_available: is_user_available,
				tier: tier,
			};
			matches_vec.push(ss2);
		}

		// Sort matches
		//matches_vec.sort_by(|a, b| b.tier.cmp(&a.tier)); // this is a reverse sort so the line below may be overly complex?
		matches_vec.sort_by(|a, b| a.tier.cmp(&b.tier));

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
