use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::matchcandidates::MatchCandidate;
use crate::models::matchcandidates::Pool;
use crate::models::users::LoggedUser;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub projectname: String,
}

pub async fn get_all_matches(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
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

fn query(pool: web::Data<Pool>) -> Result<Vec<MatchCandidate>, crate::errors::ServiceError> {
	use crate::schema::matchcandidates::dsl::matchcandidates;
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = matchcandidates.load::<MatchCandidate>(conn)?;

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

pub async fn get_matches_by_params(
	querydata: web::Json<QueryData>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	println!("\nGetting matches by parameters");
	let res = web::block(move || query_by_params(querydata, pool)).await;
	match res {
		Ok(matches) => Ok(HttpResponse::Ok().json(&matches)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_by_params(
	querydata: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<Vec<MatchCandidate>, crate::errors::ServiceError> {
	use crate::schema::matchcandidates::dsl::matchcandidates;
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = matchcandidates.load::<MatchCandidate>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.available == true);
	items.retain(|x| x.projectname == querydata.projectname);

	if items.is_empty() == false {
		println!("\nGot some matches.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}
