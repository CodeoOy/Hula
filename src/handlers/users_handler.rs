use crate::errors::{ForbiddenType, ServiceError};
use crate::models::users::{LoggedUser, Pool, UserSkill};
use crate::repositories::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use serde::{Deserialize, Serialize};
use log::trace;

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub id: String,
	pub firstname: String,
	pub lastname: String,
	pub available: bool,
	pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct UserSkillData {
	pub years: Option<f64>,
	pub email: String,
	pub user_id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
pub struct Favorite {
	pub email: String,
	pub project_id: uuid::Uuid,
	pub user_id: uuid::Uuid,
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
	pub skills: Vec<SkillDTO>,
}
#[derive(Serialize, Debug)]
pub struct SkillDTO {
	pub id: uuid::Uuid,
	pub user_id: uuid::Uuid,
	pub skill_id: uuid::Uuid,
	pub skillscopelevel_id: uuid::Uuid,
	pub years: Option<f64>,
	pub skill_label: String,
}

pub async fn get_all(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all users: logged_user = {:#?}", &_logged_user);
	let res = web::block(move || users_repository::query_all(&pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_user(
	uuid_data: web::Path<String>,
	payload: web::Json<QueryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Updating a user: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &logged_user);

	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || users_repository::query_update(uuid, payload.firstname.clone(), payload.lastname.clone(), payload.available, payload.email.clone(), pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_skill(
	uuid_data: web::Path<String>,
	payload: web::Json<UserSkill>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Adding a user skill: uuid_data = {:#?} payload = {:#?} logged_user = {:#?}", &uuid_data, &payload, &logged_user);

	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || userskills_repository::query_add_skill(uuid, payload, pool, logged_user.email)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_userskill(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Deleting a user skill: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &logged_user);
	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || userskills_repository::query_delete_userskill(uuid, pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_uuid(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting a user by uuid: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &_logged_user);
	let res = web::block(move || query_one(uuid_data.into_inner(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_one(uuid_data: String, pool: web::Data<Pool>) -> Result<UserDTO, ServiceError> {
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;

	let user = users_repository::get(uuid_query, &pool)?;
	let allskills = skills_repository::query_all_skills(&pool)?;
	let mut skills_dto: Vec<SkillDTO> = Vec::new();
	let user_skills = userskills_repository::query_belong_to_user(&user, &pool)?;

	for user_skill in user_skills.iter() {
		let mut allskills_iter = allskills.iter(); // Iterator might cause problems when there are many skills
		let skilldata = SkillDTO {
			id: user_skill.id,
			user_id: user_skill.user_id,
			skill_id: user_skill.skill_id,
			skillscopelevel_id: user_skill.skillscopelevel_id,
			years: user_skill.years,
			skill_label: String::from(
				allskills_iter
					.find(|&x| x.id == user_skill.skill_id)
					.unwrap()
					.label
					.clone(),
			),
		};
		skills_dto.push(skilldata);
	}
	let data = UserDTO {
		id: user.id,
		isadmin: user.isadmin,
		ispro: user.ispro,
		available: user.available,
		email: user.email,
		firstname: user.firstname,
		lastname: user.lastname,
		skills: skills_dto,
	};
	if data.id.is_nil() == false {
		return Ok(data.into());
	}
	Err(ServiceError::Empty)
}

pub async fn delete_user(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Deleting a user: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &logged_user);

	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || users_repository::query_delete_user(uuid, pool)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_year(
	uuid_data: web::Path<String>,
	payload: web::Json<UserSkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Updating a user years: uuid_data = {:#?} payload = {:#?} logged_user = {:#?}", &uuid_data, &payload, &logged_user);

	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || userskills_repository::query_update_year(uuid, payload.user_id.clone(), payload.years, pool, logged_user.email)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_favorite_project(
	uuid_data: web::Path<String>,
	payload: web::Json<Favorite>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Adding a favorite project: uuid_data = {:#?} payload = {:#?} logged_user = {:#?}", &uuid_data, &payload, &logged_user);

	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || userfavorites_repository::query_add_favorite_project(uuid, payload.project_id.clone(), pool, logged_user.email)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_favorite_project(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Delete a favorite project: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &logged_user);

	let uuid = uuid_data.into_inner();

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || userfavorites_repository::query_delete_favorite_project(uuid, pool)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

