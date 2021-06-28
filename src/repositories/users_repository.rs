use crate::errors::ServiceError;
use crate::models::users::{Pool, User};
use actix_web::web;
use diesel::{prelude::*, PgConnection};

pub fn query_all(pool: &web::Data<Pool>) -> Result<Vec<User>, ServiceError> {
	use crate::schema::users::dsl::users;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = users.load::<User>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn get_by_email(q_email: String, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = users.filter(email.eq(&q_email)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		return Ok(user);
	}
	Err(ServiceError::Empty)
}

pub fn get(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = users.filter(id.eq(&q_id)).load::<User>(conn)?;
	if let Some(user) = items.pop() {
		return Ok(user);
	}
	Err(ServiceError::Empty)
}


pub fn query_update(
	uuid_data: String,
	first_name: String,
	last_name: String,
	user_available: bool,
	email: String,
	pool: web::Data<Pool>,
) -> Result<User, crate::errors::ServiceError> {
	use crate::schema::users::dsl::{available, firstname, id, lastname, updated_by, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let mut items = diesel::update(users)
		.filter(id.eq(uuid_query))
		.set((
			firstname.eq(first_name),
			lastname.eq(last_name),
			available.eq(user_available),
			updated_by.eq(email),
		))
		.load::<User>(conn)?;
	if let Some(user_res) = items.pop() {
		return Ok(user_res.into());
	}
	Err(ServiceError::Unauthorized)
}


/*
fn query_one(uuid_data: String, pool: web::Data<Pool>) -> Result<UserDTO, crate::errors::ServiceError> {
	use crate::schema::skills::dsl::skills;
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let user = users.filter(id.eq(uuid_query)).get_result::<User>(conn)?; // Make a prettier error check, this produces 500
	let allskills = skills.load::<Skill>(conn)?;
	let mut skills_dto: Vec<SkillDTO> = Vec::new();
	let user_skills = UserSkill::belonging_to(&user).load::<UserSkill>(conn)?;
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
*/

pub fn query_delete_user(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::users::dsl::id;
	use crate::schema::users::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(users.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
