use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, User};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub uid: String
}

pub async fn get_all(
	payload: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting all users");
	let res = web::block(move || query(payload.into_inner(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query(
	payload: QueryData,
	pool: web::Data<Pool>,
) -> Result<Vec<User>, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{users};
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = users
		.load::<User>(conn)?;
	/*
	if let Some(query_res) = items.as_chunks() {
		println!("\nQuery successful.\n");
		return Ok(query_res.into());
	}
	*/
	return Ok(items);
	Err(ServiceError::Unauthorized)
}
