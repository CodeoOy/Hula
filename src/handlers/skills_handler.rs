use actix_web::{error::BlockingError, web, HttpResponse};
use serde::Deserialize;
use log::trace;

use crate::errors::{ForbiddenType, ServiceError};
use crate::models::skills::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Deserialize, Debug)]
pub struct SkillData {
	pub label: String,
	pub skillcategory_id: uuid::Uuid,
	pub skillscope_id: uuid::Uuid,
}
#[derive(Deserialize, Debug)]
pub struct CategoryData {
	pub label: String,
	pub parent_id: Option<uuid::Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct ScopeData {
	pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct ScopeLevelData {
	pub label: String,
	pub skillscope_id: uuid::Uuid,
	pub percentage: Option<i32>,
}

pub async fn get_all_skills(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all skills: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skills_repository::query_all_skills(&pool)).await;

	match res {
		Ok(skills) => {
			if skills.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&skills));
			}
			Err(ServiceError::Empty)
		},
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_skillscopes(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting skill scopes: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skillscopes_repository::query_skill_scopes(&pool)).await;

	match res {
		Ok(skillscopes) => {
			if skillscopes.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&skillscopes));
			}
			Err(ServiceError::Empty)
		},
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_skill_levels(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting skill levels: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skillscopelevels_repository::query_skill_levels(&pool)).await;

	match res {
		Ok(skill_levels) => {
			if skill_levels.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&skill_levels));
			}
			Err(ServiceError::Empty)
		},
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_skill_categories(
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting skill categories: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skillcategories_repository::query_skill_categories(pool)).await;

	match res {
		Ok(categories) => {
			if categories.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&categories));
			}
			Err(ServiceError::Empty)
		},
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill_category(
	categorydata: web::Json<CategoryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Creating a skill category: categorydata = {:#?} logged_user = {:#?}", &categorydata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || skillcategories_repository::create_skill_category(categorydata.label.clone(), categorydata.parent_id, logged_user.email, &pool)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_skill_category(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Deleting a skill category: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skillcategories_repository::delete_skill_category(id, &pool)).await;
	match res {
		Ok(deleted) => {
			if deleted > 0 {
				return Ok(HttpResponse::Ok().finish());
			}
			Err(ServiceError::Gone)
		},
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill(
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Creating a skill: skilldata = {:#?} logged_user = {:#?}", &skilldata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || skills_repository::create_skill(skilldata.label.clone(), skilldata.skillcategory_id, skilldata.skillscope_id, logged_user.email, &pool)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_skill(
	uuid_data: web::Path<String>,
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Creating a skill: skilldata = {:#?} logged_user = {:#?}", &skilldata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skills_repository::update_skill(id, skilldata.label.clone(), skilldata.skillcategory_id, logged_user.email, &pool)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill_scope(
	scopedata: web::Json<ScopeData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Creating a skill scope: scopedata = {:#?} logged_user = {:#?}", &scopedata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || skillscopes_repository::create_skill_scope(scopedata.label.clone(), logged_user.email, &pool)).await;
	match res {
		Ok(skill_scope) => Ok(HttpResponse::Ok().json(&skill_scope)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill_scope_level(
	scopeleveldata: web::Json<ScopeLevelData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Creating a skill scope level: scopeleveldata = {:#?} logged_user = {:#?}", &scopeleveldata, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let res = web::block(move || skillscopelevels_repository::create_skill_scope_level(scopeleveldata.label.clone(), scopeleveldata.percentage, scopeleveldata.skillscope_id.clone(), logged_user.email, &pool)).await;
	match res {
		Ok(skill_scope_level) => Ok(HttpResponse::Ok().json(&skill_scope_level)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_skill(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Deleting a skill: uuid_data = {:#?} logged_user = {:#?}", &uuid_data, &logged_user);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::Forbidden(ForbiddenType::AdminRequired));
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skills_repository::delete_skill(id, &pool)).await;
	match res {
		Ok(deleted) => {
			if deleted > 0 {
				return Ok(HttpResponse::Ok().finish());
			}
			Err(ServiceError::Gone)
		},
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

