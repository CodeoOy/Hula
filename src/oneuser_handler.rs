use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, User};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub uid: String
}

pub async fn get_by_uuid(
	uuid_data: web::Json<QueryData>,
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
	// First parameter is the struct written at the start of this file.
	uuid_data: QueryData,
	// The second parameter is still a bit of a mystery, need to look into that
	pool: web::Data<Pool>,
	// First return value is the type of data we're fetching. These are found in models.rs but a generic
	// or raw result should do to, we don't want to write a new struct for every single query return
) -> Result<User, crate::errors::ServiceError> {
	// Line below imports the schema
	use crate::schema::users::dsl::{uid, users};
	// Line below declares the connection
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data.uid)?;
	// Declaration below does the actual search from db
	let mut items = users
        .filter(uid.eq(uuid_query))
		// This does the query but what's with the question mark?
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nQuery successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}
