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
	let upload_path = get_upload_path(logged_user.uid)?;

/*
	let mut upload_path = std::env::var("USER_UPLOAD_PATH").unwrap();

	if upload_path.chars().last().unwrap() != '/' {
		upload_path.push_str("/");	
	}

	upload_path.push_str(&logged_user.uid.to_string());
	trace!("path={}", &upload_path);

	let existing = std::path::Path::new(&upload_path).exists();
	if existing == false {
		let _ = std::fs::create_dir_all(&upload_path)?;
	}
*/


	let mut tempfilename = String::new();
	while let Ok(Some(mut field)) = payload.try_next().await {
		match field.content_disposition() {
			None => trace!("content disposition not set"),
			Some(content_disposition) => match content_disposition.get_filename() {
				None => continue, // ignore non-file field
				Some(filename) => {
					tempfilename = filename.to_string();
				}
			},
		}

		let filepath = format!(
			"{}/{}",
			upload_path,
			sanitize_filename::sanitize(&tempfilename)
		);
		trace!("filepath={}", &filepath);

		let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();

		while let Some(chunk) = field.next().await {
			let data = chunk.unwrap();
			f = web::block(move || f.write_all(&data).map(|_| f)).await?;
		}
	}

	web::block(move || useruploads_repository::create_file(logged_user.uid.clone(), tempfilename.clone(), logged_user.email.clone(), &pool))
	.await?;

	Ok(HttpResponse::Ok().into())
}

pub async fn delete_file(
	uuid_data: web::Path<String>, 
	logged_user: LoggedUser,
	pool: web::Data<Pool>) 
-> Result<HttpResponse, ServiceError> {
	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let upload_path = get_upload_path(logged_user.uid);
	let _upload_path = match upload_path {
		Ok(x) => x,
		Err(_) => 
			return Err(ServiceError::InternalServerError)
	};

	/*
	let res = useruploads_repository::get_by_fileid(id, &pool).await;

	let filepath = format!(
		"{}/{}",
		upload_path,
		sanitize_filename::sanitize(&tempfilename)
	);
*/

	/*
	let filename = format!("./{}.pdf", filename);
		fs::remove_file(filename)?;
	
	*/

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

fn get_upload_path(uid: uuid::Uuid) -> std::io::Result<String> {
	let mut upload_path = std::env::var("USER_UPLOAD_PATH").unwrap();

	if upload_path.chars().last().unwrap() != '/' {
		upload_path.push_str("/");	
	}

	upload_path.push_str(&uid.to_string());
	trace!("path={}", &upload_path);

	let existing = std::path::Path::new(&upload_path).exists();
	if existing == false {
		let _ = std::fs::create_dir_all(&upload_path)?;
	}

	Ok(upload_path)
}
