use actix_web::{error::BlockingError, web, HttpResponse};
use serde::Deserialize;
use log::trace;

use crate::errors::ServiceError;
use crate::models::matchcandidates::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub projectname: String,
}

pub async fn get_all_matches(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all matches: logged_user={:#?}", &_logged_user);
	let res = web::block(move || matches_repository::query(&pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_matches_by_params(
	querydata: web::Json<QueryData>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting matches by params: querydata = {:#?} logged_user={:#?}", &querydata, &_logged_user);
	let res = web::block(move || matches_repository::query_by_params(querydata.projectname.clone(), &pool)).await;
	match res {
		Ok(matches) => Ok(HttpResponse::Ok().json(&matches)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

