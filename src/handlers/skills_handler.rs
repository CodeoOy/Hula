use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::{Deserialize};

use crate::models::users::LoggedUser;
use crate::errors::ServiceError;
use crate::models::skills::{Pool, Skill, SkillCategory, SkillScope, SkillScopeLevel};

#[derive(Deserialize, Debug)]
pub struct SkillData {
	pub email: String,
	pub label: String,
	pub category_id: uuid::Uuid, 
	pub skillscope_id: uuid::Uuid, 
}
#[derive(Deserialize, Debug)]
pub struct CategoryData {
	pub email: String,
	pub label: String,
	pub parent_id: Option<uuid::Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct ScopeData {
	pub email: String,
	pub label: String,
	pub category_id: uuid::Uuid, 
}

#[derive(Deserialize, Debug)]
pub struct ScopeLevelData {
	pub email: String,
	pub label: String,
	pub skillscope_id: uuid::Uuid,
	pub percentage: Option<i32>,
}

pub async fn get_all_skills(
	pool: web::Data<Pool>,
	_logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_all_skills(pool)).await;

	match res {
		Ok(skills) => Ok(HttpResponse::Ok().json(&skills)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_all_skills(pool: web::Data<Pool>) -> Result<Vec<Skill>, crate::errors::ServiceError> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skills.load::<Skill>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all skills.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn get_skillscopes(
	pool: web::Data<Pool>,
	_logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_skillscopes(pool)).await;

	match res {
		Ok(skillscopes) => Ok(HttpResponse::Ok().json(&skillscopes)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_skillscopes(pool: web::Data<Pool>) -> Result<Vec<SkillScope>, crate::errors::ServiceError> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillscopes.load::<SkillScope>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all skillscopes.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn get_skill_levels(
	pool: web::Data<Pool>,
	_logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_skill_levels(pool)).await;

	match res {
		Ok(skill_levels) => Ok(HttpResponse::Ok().json(&skill_levels)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_skill_levels(pool: web::Data<Pool>) -> Result<Vec<SkillScopeLevel>, crate::errors::ServiceError> {
	use crate::schema::skillscopelevels::dsl::skillscopelevels;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillscopelevels.load::<SkillScopeLevel>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all skillscopelevels.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn get_skill_categories(
	pool: web::Data<Pool>,
	_logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	let res = web::block(move || query_skill_categories(pool)).await;

	match res {
		Ok(categories) => Ok(HttpResponse::Ok().json(&categories)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_skill_categories(pool: web::Data<Pool>) -> Result<Vec<SkillCategory>, crate::errors::ServiceError> {
	use crate::schema::skillcategories::dsl::skillcategories;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillcategories.load::<SkillCategory>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all categories.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn create_skill_category(
	categorydata: web::Json<CategoryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a skill category");
	let res = web::block(move || query_create_skill_category(categorydata, pool, logged_user.email)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_skill_category(
	categorydata: web::Json<CategoryData>,
	pool: web::Data<Pool>,
	email: String
) -> Result<SkillCategory, crate::errors::ServiceError> {
	use crate::schema::skillcategories::dsl::skillcategories;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_skill_category = SkillCategory {
		id: uuid::Uuid::new_v4(),
		label: categorydata.label.clone(),
		parent_id: categorydata.parent_id,
		updated_by: email
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(skillcategories)
		.values(&new_skill_category)
		.get_result::<SkillCategory>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSkill added successfully.\n");
		return Ok(new_skill_category.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn create_skill(
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a skill");
	let res = web::block(move || query_create_skill(skilldata, pool, logged_user.email)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_skill(
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
	email: String
) -> Result<Skill, crate::errors::ServiceError> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_skill = Skill {
		id: uuid::Uuid::new_v4(),
		label: skilldata.label.clone(),
		skillcategory_id: skilldata.category_id,
		skillscope_id: skilldata.skillscope_id,
		updated_by: email
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(skills)
		.values(&new_skill)
		.get_result::<Skill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSkill added successfully.\n");
		return Ok(new_skill.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn create_skill_scope(
	scopedata: web::Json<ScopeData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a skill scope");
	let res = web::block(move || query_create_skill_scope(scopedata, pool, logged_user.email)).await;
	match res {
		Ok(skill_scope) => Ok(HttpResponse::Ok().json(&skill_scope)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_skill_scope(
	scopedata: web::Json<ScopeData>,
	pool: web::Data<Pool>,
	email: String
) -> Result<SkillScope, crate::errors::ServiceError> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_scope = SkillScope {
		id: uuid::Uuid::new_v4(),
		label: scopedata.label.clone(),
		updated_by: email
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(skillscopes)
		.values(&new_scope)
		.get_result::<SkillScope>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSkill scope added successfully.\n");
		return Ok(new_scope.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn create_skill_scope_level(
	scopeleveldata: web::Json<ScopeLevelData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a skill scope level");
	let res = web::block(move || query_create_skill_scope_level(scopeleveldata, pool, logged_user.email)).await;
	match res {
		Ok(skill_scope_level) => Ok(HttpResponse::Ok().json(&skill_scope_level)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create_skill_scope_level(
	scopeleveldata: web::Json<ScopeLevelData>,
	pool: web::Data<Pool>,
	email: String
) -> Result<SkillScopeLevel, crate::errors::ServiceError> {
	use crate::schema::skillscopelevels::dsl::{skillscope_id, index, skillscopelevels};
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let mut currentindex = 0;
	let latestlevel = skillscopelevels
		.filter(skillscope_id.eq(scopeleveldata.skillscope_id))
		.order_by(index.desc())
		.limit(1)
		.load::<SkillScopeLevel>(conn)?;
	if latestlevel.len() > 0 {
		currentindex = latestlevel[0].index;
	}
	let new_scope_level = SkillScopeLevel {
		id: uuid::Uuid::new_v4(),
		label: scopeleveldata.label.clone(), // TODO: This is unique, sanitize field or handle error more gracefully
		skillscope_id: scopeleveldata.skillscope_id,
		index: currentindex + 1,
		percentage: scopeleveldata.percentage,
		updated_by: email
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(skillscopelevels)
		.values(&new_scope_level)
		.get_result::<SkillScopeLevel>(conn);
	if rows_inserted.is_ok() {
		println!("\nSkill scope level added successfully.\n");
		return Ok(new_scope_level.into());
	}
	Err(ServiceError::Unauthorized)
}