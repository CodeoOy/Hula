use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, Pool};
use crate::models::users::{SlimUser, User};
// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
	pub email: String,
	pub password: String,
	pub id: String,
}

pub async fn register_user(
	user_data: web::Json<UserData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query(user_data.into_inner(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query(
	user_data: UserData,
	pool: web::Data<Pool>
) -> Result<SlimUser, crate::errors::ServiceError> {
	use crate::schema::invitations::dsl::{email, id, invitations};
	use crate::schema::users::dsl::users;
	let invitation_id = uuid::Uuid::parse_str(&user_data.id)?;

	let conn: &PgConnection = &pool.get().unwrap();
	invitations
		.filter(id.eq(invitation_id))
		.filter(email.eq(&user_data.email))
		.load::<Invitation>(conn)
		.map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
		.and_then(|mut result| {
			if let Some(invitation) = result.pop() {
				// if invitation is not expired
				if invitation.expires_at > chrono::Local::now().naive_local() {
					// try hashing the password, else return the error that will be converted to ServiceError
					let password: String = user_data.password;
					let user =
						User::from_details(invitation.email, password, invitation.first_name, invitation.last_name);
					let inserted_user: User = diesel::insert_into(users).values(&user).get_result(conn)?;
					dbg!(&inserted_user);
					return Ok(inserted_user.into());
				}
			}
			Err(ServiceError::BadRequest("Invalid Invitation".into()))
		})
}
