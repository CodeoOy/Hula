use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool};
use crate::repositories::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub id: String,
	pub firstname: String,
	pub lastname: String,
	pub is_hidden: bool,
	pub is_employee: bool,
	pub isadmin: bool,
	pub email: String,
	pub main_upload_id: Option<uuid::Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct NewUserData {
	pub email: String,
	pub password: String,
	pub firstname: String,
	pub lastname: String,
}

#[derive(Deserialize, Debug)]
pub struct UserSkillData {
	pub skill_id: uuid::Uuid,
	pub skillscopelevel_id: Option<uuid::Uuid>,
	pub years: Option<f64>,
	pub user_id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
pub struct UserReservationData {
	pub description: String,
	pub begin_time: Option<chrono::NaiveDate>,
	pub end_time: Option<chrono::NaiveDate>,
	pub percentage: Option<i32>,
	pub user_id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
pub struct FavoriteData {
	pub email: String,
	pub project_id: uuid::Uuid,
	pub user_id: uuid::Uuid,
}

#[derive(Serialize, Debug)]
pub struct UserDTO {
	pub id: uuid::Uuid,
	pub isadmin: bool,
	pub is_hidden: bool,
	pub is_employee: bool,
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
	pub skillscopelevel_id: Option<uuid::Uuid>,
	pub years: Option<f64>,
	pub skill_label: String,
}

#[derive(Deserialize, Debug)]
pub struct ForgotPasswordData {
	pub email: String,
	pub password: String,
	pub id: uuid::Uuid,
}

pub async fn get_all(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all users: logged_user = {:#?}", &_logged_user);
	let res = web::block(move || users_repository::query_all(&pool)).await;

	match res {
		Ok(users) => {
			if users.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&users));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
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
	trace!(
		"Updating a user: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		users_repository::update(
			id,
			payload.firstname.clone(),
			payload.lastname.clone(),
			payload.is_hidden,
			payload.is_employee,
			payload.isadmin,
			payload.email.clone(),
			payload.main_upload_id,
			&pool,
		)
	})
	.await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_skill(
	uuid_data: web::Path<String>,
	payload: web::Json<UserSkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a user skill: uuid_data = {:#?} payload = {:#?} logged_user = {:#?}",
		&uuid_data,
		&payload,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		userskills_repository::add_skill(
			id,
			payload.skill_id,
			payload.skillscopelevel_id,
			payload.years,
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_userskill(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a user skill: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);
	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || userskills_repository::delete_userskill(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_skill(
	id: web::Path<String>,
	payload: web::Json<UserSkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating user skills: id = {:#?} payload = {:#?} logged_user = {:#?}",
		&id,
		&payload,
		&logged_user
	);

	let skill_id = uuid::Uuid::parse_str(&id.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != payload.user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		userskills_repository::update_skill(
			skill_id,
			payload.user_id.clone(),
			payload.skillscopelevel_id,
			payload.years,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(userskill) => Ok(HttpResponse::Ok().json(&userskill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_reservations(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting user reservations by their id uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let user = users_repository::get(uuid_query, &pool)?;
	let res = web::block(move || userreservations_repository::query_belong_to_user(&user.into(), &pool)).await;

	match res {
		Ok(userreservations) => {
			if userreservations.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&userreservations));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_reservation(
	uuid_data: web::Path<String>,
	payload: web::Json<UserReservationData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a user reservation: uuid_data = {:#?} payload = {:#?} logged_user = {:#?}",
		&uuid_data,
		&payload,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	// todo: create a macro to simplify this
	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		userreservations_repository::add_skill(
			id,
			payload.description.clone(),
			payload.begin_time,
			payload.end_time,
			payload.percentage,
			logged_user.email,
			&pool,
		)
	})
	.await;

	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_reservation(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a user reservation: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);
	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || userreservations_repository::delete_reservation(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_reservation(
	id: web::Path<String>,
	payload: web::Json<UserReservationData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating user skills: id = {:#?} payload = {:#?} logged_user = {:#?}",
		&id,
		&payload,
		&logged_user
	);

	let reservation_id = uuid::Uuid::parse_str(&id.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != payload.user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		userreservations_repository::update_reservation(
			reservation_id,
			payload.description.clone(),
			payload.begin_time,
			payload.end_time,
			payload.percentage,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(userreservation) => Ok(HttpResponse::Ok().json(&userreservation)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_uuid(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting a user by uuid: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&_logged_user
	);
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
		is_hidden: user.is_hidden,
		is_employee: user.is_employee,
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
	trace!(
		"Deleting a user: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || users_repository::delete_user(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_favorite_project(
	uuid_data: web::Path<String>,
	payload: web::Json<FavoriteData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a favorite project: uuid_data = {:#?} payload = {:#?} logged_user = {:#?}",
		&uuid_data,
		&payload,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		userfavorites_repository::add_favorite_project(id, payload.project_id.clone(), logged_user.email, &pool)
	})
	.await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_favorite_project(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete a favorite project: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	if logged_user.isadmin == false && logged_user.uid != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || userfavorites_repository::delete_favorite_project(id, &pool)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_password(
	payload: web::Json<ForgotPasswordData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Resetting password for: email = {:#?}", &payload.email,);

	let id = reset_requests_repository::get_by_reset_request(payload.id.clone(), &pool);
	match id {
		Ok(_) => println!("Reset request found"),
		Err(_) => return Err(ServiceError::InternalServerError),
	}

	let res =
		web::block(move || users_repository::set_password(payload.email.clone(), payload.password.clone(), &pool))
			.await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
