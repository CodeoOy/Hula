use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::offers::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct OfferData {
	pub user_id: uuid::Uuid,
	pub project_id: uuid::Uuid,
	pub sold: bool,
	pub comments: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OfferUpdateData {
	pub sold: bool,
	pub comments: Option<String>,
}

pub async fn get_all_offers(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all offers: logged_user={:#?}", &_logged_user);
	let res = web::block(move || offers_repository::query_get_all_offers(&pool)).await;

	match res {
		Ok(offers) => {
			if offers.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&offers));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_offer(
	payload: web::Json<OfferData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Posting offer: logged_user={:#?}", &logged_user);
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}
	let res = web::block(move || {
		offers_repository::query_add_offer(
			payload.user_id,
			payload.project_id,
			payload.comments.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(offer) => Ok(HttpResponse::Ok().json(&offer)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_offer(
	oid: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all offers: logged_user={:#?}", &logged_user);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}
	let id = uuid::Uuid::parse_str(&oid.into_inner())?;

	let res = web::block(move || offers_repository::query_delete_offer(id, &pool)).await;
	match res {
		Ok(offer) => Ok(HttpResponse::Ok().json(&offer)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_offer(
	oid: web::Path<String>,
	pool: web::Data<Pool>,
	payload: web::Json<OfferUpdateData>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Updating offer: logged_user={:#?}", &logged_user);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}
	let id = uuid::Uuid::parse_str(&oid.into_inner())?;
	let res = web::block(move || {
		offers_repository::query_update_offer(
			id,
			payload.sold,
			payload.comments.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(offer) => Ok(HttpResponse::Ok().json(&offer)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
