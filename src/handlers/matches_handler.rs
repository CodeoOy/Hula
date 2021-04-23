/* pub mod matches_handler; */

use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::{Pool};

use crate::models2::projectskill::{ProjectSkill};

pub async fn get_all_matches(
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting all matches");
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
) -> Result<Vec<ProjectSkill>, crate::errors::ServiceError> {
	use crate::schema::projectskills::dsl::{projectskills};
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projectskills
		.load::<ProjectSkill>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all projectskills.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}
