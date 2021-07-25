use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};
use crate::models::users::Pool;

use crate::models::projectmatches::ProjectMatch;
use crate::models::projects::Project;


pub fn find_by_projects(projects: &Vec<Project>, pool: &web::Data<Pool>) -> Result<Vec<Vec<ProjectMatch>>, Error> {
	use crate::schema::projectmatches::dsl::{user_last_name, user_first_name};

	let conn: &PgConnection = &pool.get().unwrap();

	let posts = ProjectMatch::belonging_to(projects)
		.order((user_last_name.asc(), user_first_name.asc()))
		.load::<ProjectMatch>(conn)?
		.grouped_by(&projects);
	
	Ok(posts)
}
