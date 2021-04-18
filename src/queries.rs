use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, User};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub uuid: String,
}

pub async fn get_by_uuid(
	uuid_data: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	let res = web::block(move || query(uuid_data.into_inner(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)), // Not user, something else?
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query(
	uuid_data: QueryData,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::users;
	//use crate::schema::users::dsl::{hash, users};
	let uuid_query = uuid::Uuid::parse_str(&uuid_data.uuid)?;
	let conn: &PgConnection = &pool.get().unwrap();
	let user_res = users
        .filter(uid.eq(uuid_query))
		.load::<User>(conn)?;
        //.first::<QueryData>(conn)
    Ok(user_res.into())
}
