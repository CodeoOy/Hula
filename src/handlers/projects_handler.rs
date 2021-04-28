use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::projects::{Pool, Project};

pub async fn get_all_projects(
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting all projects");
	let res = web::block(move || query(pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query(
	pool: web::Data<Pool>,
) -> Result<Vec<Project>, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::{projects};
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projects
		.load::<Project>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all projects.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}
