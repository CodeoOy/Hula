use actix_web::web;
use derive_more::Display;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::sql_types::Uuid;
use diesel::{prelude::*, PgConnection};
use serde::{Serialize, Deserialize};

use crate::models::skills::{Pool};
use crate::models::projects::{Project};

use crate::repositories::{projects_repository, projectneeds_repository, projectneedskills_repository, projectskills_repository, skills_repository, skillscopelevels_repository};

/*
#[derive(Debug, Display, Serialize)]
pub enum ScopeLevelSwapDirection {
	Better,
	Worse,
}
*/

#[derive(Deserialize, Debug)]
pub struct ProjectStructure {
	pub name: String,
	pub is_hidden: bool,
	pub needs: Vec<ProjectStructureNeed>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectStructureNeed {
	pub label: String,
	pub count_of_users: i32,
	pub begin_time: chrono::NaiveDate,
	pub end_time: Option<chrono::NaiveDate>,
	pub percentage: Option<i32>,
	pub skills: Vec<ProjectStructureNeedSkill>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectStructureNeedSkill {
	pub skill_label: String,
	pub skillscopelevel_label: Option<String>,
	pub min_years: Option<f64>,
	pub max_years: Option<f64>,
	pub mandatory: bool,
}

pub fn test_project_structure_equals(
	id: uuid::Uuid,
	project: ProjectStructure,
	pool: web::Data<Pool>,
) -> Result<bool, Error> {
	
	let db_project = projects_repository::query_one(id, &pool)?;
	
	if db_project.name != project.name 
		|| db_project.is_hidden != project.is_hidden {
			return Ok(false);
	}

	let db_skills = projectskills_repository::find_by_projects(&vec!(db_project), &pool)?;
	let db_skills = &db_skills[0];

	let mut skills_count = 0;

	for need in project.needs.iter() {
		skills_count += need.skills.len();

		//db_skills.iter().filter(|x| x.pn_id);		
	}

/*
	let db_needs = projectneeds_repository::query_project_needs(&pool, id)?;
	if db_needs.len() != project.needs.len() { return Ok(false); }

	for need in project.needs.iter() {
		let db_need = db_needs.iter().filter(|x| x.label == need.label).next();
		if db_need.is_none() { return Ok(false); }

		let db_need = db_need.unwrap();
		if db_need.count_of_users != need.count_of_users ||
			db_need.begin_time != need.begin_time ||
			db_need.end_time != need.end_time ||
			db_need.percentage != need.percentage {
				return Ok(false);
		}

		let db_skills = projectneedskills_repository::query_projectneedskills(db_need.id, &pool)?;
		if db_skills.len() != need.skills.len() { return Ok(false); }

		for skill in need.skills.iter() {
			let db_skill = db_skills.iter().filter(|x| x.label == need.label).next();
			if db_level.is_none() { return Ok(false); }
	
		}

		let skill_match = need.skills.iter().all(|x| db_skills.iter().any(|y| 
			x.skill_label == y.label &&
			x.skill == y.count_of_users &&
			x.begin_time == y.begin_time &&
			x.end_time == y.end_time &&
			x.percentage == y.percentage
		));
	}


	if need_match == false { return Ok(false); }
*/
/*
	pub skillscopelevel_label: Option<String>,
	pub min_years: Option<f64>,
	pub max_years: Option<f64>,
	pub mandatory: bool,

*/


	Ok(true)
}

pub fn save_project_structure(
	id: uuid::Uuid,
	project: ProjectStructure,
	pool: web::Data<Pool>,
	email: String,
) -> Result<(), Error> {

	let conn: &PgConnection = &pool.get().unwrap();

	conn.transaction::<_, Error, _>(|| {

		projects_repository::update_project(
			id,
			project.name.clone(),
			project.is_hidden,
			email.clone(),
			&pool,
		)?;

		projectneeds_repository::delete_projectneeds_by_project(id, &pool)?;	

		for need in project.needs.iter() {
			let need_res = projectneeds_repository::create_projectneed(
				id,
				need.count_of_users,
				need.percentage,
				need.begin_time,
				need.end_time,
				email.clone(),
				&pool)?;	

			for skill in need.skills.iter() {
				let db_skill = skills_repository::get_skill_by_label(skill.skill_label.clone(), &pool)?;

				let mut skill_scope_level_id :Option<uuid::Uuid> = None;
				if let Some(scopelabel) = skill.skillscopelevel_label.clone() {
					let db_skill_scope_level = skillscopelevels_repository::get_skill_level_by_label(scopelabel, &pool)?;
					skill_scope_level_id = Some(db_skill_scope_level.id);
				}

				projectneedskills_repository::create_projectneedskill(
					need_res.id,
					db_skill.id,
					skill_scope_level_id,
					skill.min_years,
					skill.max_years,
					email.clone(),
					skill.mandatory,
					&pool)?;	
			}
		}


			Ok(())

	})?;

	Ok(())

		/*
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
	*/
}
