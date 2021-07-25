use crate::models::users::Pool;
use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};

use crate::models::projects::Project;
use crate::models::projectskills::ProjectSkill;

pub fn find_by_projects(projects: &Vec<Project>, pool: &web::Data<Pool>) -> Result<Vec<Vec<ProjectSkill>>, Error> {
	use crate::schema::projectskills::dsl::skill_label;

	let conn: &PgConnection = &pool.get().unwrap();

	let skills = ProjectSkill::belonging_to(projects)
		.order(skill_label.asc())
		.load::<ProjectSkill>(conn)?
		.grouped_by(&projects);

	Ok(skills)
}
