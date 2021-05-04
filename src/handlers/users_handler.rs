use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::{Serialize, Deserialize};

use crate::errors::ServiceError;
use crate::models::users::{Pool, User, Skill};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub uid: String,
	pub firstname: String,
	pub lastname: String,
	pub available: bool
}
#[derive(Serialize, Debug)]
pub struct UserDTO {
	pub id: uuid::Uuid,
	pub isadmin: bool,
	pub ispro: bool,
	pub available: bool,
	pub email: String,
	pub firstname: String,
	pub lastname: String,
	pub skills: Vec<Skill>,
}

pub async fn get_all(
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting all users");
	let res = web::block(move || query_all(pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_all(
	pool: web::Data<Pool>,
) -> Result<Vec<User>, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{users};
	let conn: &PgConnection = &pool.get().unwrap();
	let items = users
		.load::<User>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all users.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}


pub async fn update_user(
	uuid_data: web::Path<String>,
	payload: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nUpdating user");
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
	let res = web::block(move || query_one(uuid_data.into_inner(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

/*
fn query(
	uuid_data: String,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = users
        .filter(id.eq(uuid_query))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nQuery successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}
*/


fn query_one(
	uuid_data: String,
	pool: web::Data<Pool>,
) -> Result<UserDTO, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let user = users.filter(id.eq(uuid_query)).get_result::<User>(conn)?; // Make a prettier error check, this produces 500
	let skills = Skill::belonging_to(&user)
		.load::<Skill>(conn)?;
	let data = UserDTO {
		id: user.id,
		isadmin: user.isadmin,
		ispro: user.ispro,
		available: user.available,
		email: user.email,
		firstname: user.firstname,
		lastname: user.lastname,
		skills: skills,
	};
	if data.id.is_nil() == false {
		println!("\nQuery successful.\n");
		return Ok(data.into());
	}
	Err(ServiceError::Empty)
}

fn query_update(
	uuid_data: String,
	userdata: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{users, id, firstname, lastname, available};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	//let testdata = String::from(userdata.into_inner());
	let mut items = diesel::update(users)
		.filter(id.eq(uuid_query))
		.set((
			firstname.eq(userdata.firstname.clone()),
			lastname.eq(userdata.lastname.clone()),
			available.eq(userdata.available)
		))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nUpdate successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}

