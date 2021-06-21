use crate::errors::ServiceError;
use crate::models::skills::Skill;
use crate::models::users::{LoggedUser, Pool, User, UserFavorites, UserSkill};
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::{Deserialize, Serialize};

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

pub async fn get_all(
	pool: web::Data<Pool>, 
	_logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
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
	pool: web::Data<Pool>
) -> Result<Vec<User>, crate::errors::ServiceError> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = users.load::<User>(conn)?;
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
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let uuid = uuid_data.into_inner();

	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Unauthorized);
	}
	
	println!("\nUpdating user");
	let res = web::block(move || query_update(uuid, payload, pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_update(
	uuid_data: String,
	userdata: web::Json<QueryData>,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{available, firstname, id, lastname, updated_by, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(users)
		.filter(id.eq(uuid_query))
		.set((
			firstname.eq(userdata.firstname.clone()),
			lastname.eq(userdata.lastname.clone()),
			available.eq(userdata.available),
			updated_by.eq(userdata.email.clone()),
		))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nUpdate successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn add_skill(
	uuid_data: web::Path<String>,
	payload: web::Json<UserSkill>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let uuid = uuid_data.into_inner();

	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Unauthorized);
	}
	
	println!("Adding skill");
	let res = web::block(move || query_add_skill(uuid, payload, pool, logged_user.email)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_add_skill(
	uuid_data: String,
	skill_data: web::Json<UserSkill>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<UserSkill, crate::errors::ServiceError> {
	use crate::schema::userskills::dsl::userskills;
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let new_user_skill = UserSkill {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_query,
		skill_id: skill_data.skill_id,
		skillscopelevel_id: skill_data.skillscopelevel_id,
		years: skill_data.years,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(userskills)
		.values(&new_user_skill)
		.get_result::<UserSkill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSkill added successfully.\n");
		return Ok(new_user_skill.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn get_by_uuid(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
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

fn query_one(
	uuid_data: String, 
	pool: web::Data<Pool>
) -> Result<UserDTO, crate::errors::ServiceError> {
	use crate::schema::skills::dsl::skills;
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let user = users.filter(id.eq(uuid_query)).get_result::<User>(conn)?; // Make a prettier error check, this produces 500
	let allskills = skills.load::<Skill>(conn)?;
	let mut skills_dto: Vec<SkillDTO> = Vec::new();
	let user_skills = UserSkill::belonging_to(&user).load::<UserSkill>(conn)?;
	for user_skill in user_skills.iter() {
		println!("Got a skill");
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
		println!("\nQuery successful.\n");
		return Ok(data.into());
	}
	Err(ServiceError::Empty)
}

pub async fn delete_user(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let uuid = uuid_data.into_inner();

	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Unauthorized);
	}

	let res = web::block(move || query_delete_user(uuid, pool)).await;
	println!("\nUser deleted\n");
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_user(
	uuid_data: String, 
	pool: web::Data<Pool>
) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::users::dsl::id;
	use crate::schema::users::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(users.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}

pub async fn update_year(
	uuid_data: web::Path<String>,
	payload: web::Json<UserSkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let uuid = uuid_data.into_inner();

	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Unauthorized);
	}

	println!("\nUpdating skill's year");
	let res = web::block(move || query_update_year(uuid, payload, pool, logged_user.email)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_update_year(
	uuid_data: String,
	userdata: web::Json<UserSkillData>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<UserSkill, crate::errors::ServiceError> {
	use crate::schema::userskills::dsl::*;
	use crate::schema::userskills::dsl::{skill_id, updated_by, years};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(userskills)
		.filter(skill_id.eq(uuid_query))
		.filter(user_id.eq(userdata.user_id))
		.set((years.eq(userdata.years.clone()), updated_by.eq(email)))
		.load::<UserSkill>(conn)?;
	if let Some(user_res) = items.pop() {
		println!("\nUpdate successful.\n");
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn add_favorite_project(
	uuid_data: web::Path<String>,
	payload: web::Json<Favorite>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let uuid = uuid_data.into_inner();

	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Unauthorized);
	}

	println!("Adding favorite project");
	let res = web::block(move || query_add_favorite_project(uuid, payload, pool, logged_user.email)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_add_favorite_project(
	uuid_data: String,
	favorite_data: web::Json<Favorite>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<UserFavorites, crate::errors::ServiceError> {
	use crate::schema::userfavorites::dsl::userfavorites;
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let new_favorite = UserFavorites {
		id: uuid::Uuid::new_v4(),
		user_id: uuid_query,
		project_id: favorite_data.project_id,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(userfavorites)
		.values(&new_favorite)
		.get_result::<UserFavorites>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nFavorite added successfully.\n");
		return Ok(new_favorite.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn delete_favorite_project(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	let uuid = uuid_data.into_inner();

	if logged_user.isadmin == false && logged_user.uid.to_string() != uuid.clone() {
		return Err(ServiceError::Unauthorized);
	}

	let res = web::block(move || query_delete_favorite_project(uuid, pool)).await;
	println!("\nProject removed from favorites\n");
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_delete_favorite_project(
	uuid_data: String, 
	pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::userfavorites::dsl::*;
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(userfavorites.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}

