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

fn test_project_structure_equals(
	id: uuid::Uuid,
	project: &ProjectStructure,
	pool: &web::Data<Pool>,
) -> Result<bool, Error> {
	
	let db_project = projects_repository::query_one(id, &pool)?;
	
	if db_project.name != project.name 
		|| db_project.is_hidden != project.is_hidden {
			println!("project on eri {:#?} vs {:#?}", db_project, project);
			return Ok(false);
	}

	let db_project_skills = projectskills_repository::find_by_projects(&vec!(db_project), &pool)?;
	let db_project_skills = &db_project_skills[0];
	println!("");
	println!("***db_project_skills on: {:#?}", db_project_skills);

	let mut skills_count = 0;

	for need in project.needs.iter() {
		println!("need: {:#?}", need);
		skills_count += need.skills.len();

		for skill in need.skills.iter() {
			println!("skill: {:#?}", skill);
			let db_project_skill = db_project_skills
				.iter()
				.filter(|x| x.pn_label.clone().unwrap_or_default() == need.label && x.skill_label == skill.skill_label)
				.next();

			println!("db_project_skill: {:#?}", db_project_skill);
			if db_project_skill.is_none() { return Ok(false); }

			let db_project_skill = db_project_skill.unwrap();

			println!("db_project_skill {:#?} vs {:#?}", db_project_skill, skill);
			if db_project_skill.is_mandatory != skill.mandatory ||
				db_project_skill.required_minyears != skill.min_years ||
				db_project_skill.required_label != skill.skillscopelevel_label {
					return Ok(false);
			}
		}
	}

	println!("skills_count {} versus {}", skills_count, db_project_skills.len());
	if skills_count != db_project_skills.len() {
		return Ok(false);
	}

	println!("täytyy updateta {:#?}", id);
	Ok(true)
}

pub fn save_project_structure(
	mut id: Option<uuid::Uuid>,
	project: ProjectStructure,
	pool: &web::Data<Pool>,
	email: String,
	is_update: bool,
) -> Result<Option<uuid::Uuid>, Error> {

	if is_update == true {
		println!("tsekataan onko eri");
		let equals = test_project_structure_equals(
			id.unwrap(),
			&project,
			&pool)?;

		if equals == true {
			println!("on sama");
			return Ok(None);
		}
		println!("on eri");
	}

	if is_update == false {
		let db_project = projects_repository::create_project(project.name.clone(), email.clone(), &pool)?;
		id = Some(db_project.id);
	}

	let id = id.unwrap();
		
	println!("id on {:#?}", id);
	projects_repository::update_project(
		id,
		project.name.clone(),
		project.is_hidden,
		email.clone(),
		&pool,
	)?;

	println!("updated");

	projectneeds_repository::delete_projectneeds_by_project(id, &pool)?;	

	println!("deleted");

	for need in project.needs.iter() {
		let need_res = projectneeds_repository::create_projectneed(
			id,
			need.label.clone(),
			need.count_of_users,
			need.percentage,
			need.begin_time,
			need.end_time,
			email.clone(),
			&pool)?;	

		for skill in need.skills.iter() {
			println!("haetaan skill");
			let db_skill = skills_repository::get_skill_by_label(skill.skill_label.clone(), &pool);
			if db_skill.is_err() {
				continue;
			}
			
			let db_skill = db_skill.unwrap();

			println!("haettu skill");

			let mut skill_scope_level_id :Option<uuid::Uuid> = None;
			if let Some(scopelabel) = skill.skillscopelevel_label.clone() {
				println!("haetaan level for {}", scopelabel);

				let db_skill_scope_level = skillscopelevels_repository::get_skill_level_by_label(scopelabel, &pool);

				if db_skill_scope_level.is_err() == false {
					println!("löytyi");
					let db_skill_scope_level = db_skill_scope_level.unwrap();
					skill_scope_level_id = Some(db_skill_scope_level.id);
				}
				println!("haettu level");
				
			}

			println!("luodaan projectneedskill");
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

	Ok(Some(id))
}
