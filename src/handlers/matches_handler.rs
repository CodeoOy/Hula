use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::matchcandidates::Pool;
use crate::models::matchcandidates::MatchCandidate;

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
) -> Result<Vec<MatchCandidate>, crate::errors::ServiceError> {
	use crate::schema::matchcandidates::dsl::matchcandidates;
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = matchcandidates
		.load::<MatchCandidate>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.available == true);

	if items.is_empty() == false {
		println!("\nGot some matches.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}
