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
	/*
	users
		.filter(hash.eq(&uuid.uuid))
		.load::<Invitation>(conn)
		.map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
		.and_then(|mut result| {
			if let Some(invitation) = result.pop() {
				// if invitation is not expired
				if invitation.expires_at > chrono::Local::now().naive_local() {
					// try hashing the password, else return the error that will be converted to ServiceError
					let password: String = user_data.password;
					let user = User::from_details(invitation.email, password);
					let inserted_user: User =
						diesel::insert_into(users).values(&user).get_result(conn)?;
					dbg!(&inserted_user);
					return Ok(inserted_user.into());
				}
			}
			Err(ServiceError::BadRequest("Can't get data".into()))
		})
		*/
}


fn query(
	eml: String,
	psw: String,
	pool: web::Data<Pool>,
) -> Result<Invitation, crate::errors::ServiceError> {
	use crate::schema::invitations::dsl::invitations;

	let password: String = hash_password(&psw)?;
	let new_invitation = Invitation::from_details(eml, password);
	let conn: &PgConnection = &pool.get().unwrap();

	let inserted_invitation = diesel::insert_into(invitations)
		.values(&new_invitation)
		.get_result(conn)?;

	Ok(inserted_invitation)
}
