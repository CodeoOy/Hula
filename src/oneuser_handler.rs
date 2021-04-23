use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, User};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub payload: String
}

pub async fn update_user(
	uuid_data: web::Path<String>,
	payload: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting project by uuid");
	let res = web::block(move || query_update(uuid_data.into_inner(), payload, pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_uuid(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting user by uuid");
	let res = web::block(move || query(uuid_data.into_inner(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query(
	uuid_data: String,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{uid, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = users
        .filter(uid.eq(uuid_query))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nQuery successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}

fn query_update (
	uuid_data: String,
	userdata: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{users, uid};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	//let uuid_query = uuid::Uuid::parse_str(&userdata.payload)?;
	let mut items = users
		.filter(uid.eq(uuid_query))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nQuery successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}
