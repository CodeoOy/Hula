use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::skills::{Pool, Skill};

#[derive(Deserialize, Debug)]
pub struct SkillData {
	pub email: String,
	pub label: String,
}

pub async fn create_skill(
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a skill");
	let res = web::block(move || query_create(skilldata, pool)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_create(
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
) -> Result<Skill, crate::errors::ServiceError> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_skill = Skill {
		id: uuid::Uuid::new_v4(),
		label: skilldata.label.clone(),
		skillcategory_id: uuid::Uuid::parse_str("e9becc32-0238-4561-b341-106de1c26041")?, // Update to proper value
		skillscope_id: uuid::Uuid::parse_str("e9becc32-0238-4561-b341-106de1c26043")?, // Update to proper value
		updated_by: skilldata.email.clone(),
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