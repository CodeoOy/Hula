use crate::errors::ServiceError;
use crate::models::users::{LoggedUser};
use crate::repositories::*;
use actix_multipart::Multipart;
use actix_web::Error;
use actix_web::{error::BlockingError, web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use dotenv::dotenv;
use log::trace;
use serde::{Deserialize, Serialize};
use std::env;
//use std::fs;
use std::io::Write;
//use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct File {
	name: String,
}

pub async fn upload_file(
    mut payload: Multipart,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	dotenv::dotenv().ok();
	let mut filename = "".to_string();
	trace!("Trying to upload file");
	while let Ok(Some(mut field)) = payload.try_next().await {
		let cv_path = env::var("CV_PATH").expect("path must be set.");
		//filename = format!(".pdf");
	    filename = format!("{}.pdf", logged_user.uid.to_string());
		let filepath = format!("{}/{}", cv_path, filename);
		trace!("Uploading CV = {:#?} to path: {:?}", &filename, &cv_path);
		let mut file = web::block(|| std::fs::File::create(filepath)).await.unwrap();
		while let Some(chunk) = field.next().await {
			let data = chunk.unwrap();
			file = web::block(move || file.write_all(&data).map(|_| file)).await?;
		}
	}
	Ok(HttpResponse::Ok().json(&File { name: filename }))
}