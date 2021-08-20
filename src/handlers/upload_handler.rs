use crate::errors::ServiceError;
use crate::models::users::LoggedUser;
use crate::models::users::Pool;
use crate::repositories::*;
use actix_multipart::Multipart;
use actix_web::Error;
use actix_web::{error::BlockingError, web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use log::trace;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
	name: String,
}

pub async fn save_file(
	mut payload: Multipart,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, Error> {
	let filename = format!("{}.pdf", logged_user.uid);
	while let Ok(Some(mut field)) = payload.try_next().await {
		let filepath = format!("./{}", sanitize_filename::sanitize(&filename));

		let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();

		while let Some(chunk) = field.next().await {
			let data = chunk.unwrap();
			f = web::block(move || f.write_all(&data).map(|_| f)).await?;
		}
	}
	web::block(move || useruploads_repository::create_file(logged_user.uid, filename, logged_user.email, &pool))
		.await?;

	Ok(HttpResponse::Ok().into())
}

pub async fn delete_file(uuid_data: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || useruploads_repository::delete_file(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_userfiles(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all skills: logged_user= {:#?}", &_logged_user);

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;

	let res = web::block(move || useruploads_repository::get_by_userid(uuid_query, &pool)).await;

	match res {
		Ok(useruploads) => Ok(HttpResponse::Ok().json(&useruploads)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
