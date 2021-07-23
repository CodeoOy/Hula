use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};
use crate::models::users::Pool;

use crate::models::projectmatches::ProjectMatch;
use crate::models::projects::Project;


pub fn find_by_projects(projects: &Vec<Project>, pool: &web::Data<Pool>) -> Result<Vec<Vec<ProjectMatch>>, Error> {
	let conn: &PgConnection = &pool.get().unwrap();

	let posts = ProjectMatch::belonging_to(projects)
		.load::<ProjectMatch>(conn)?
		.grouped_by(&projects);
	
	Ok(posts)
}
