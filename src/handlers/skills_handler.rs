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
	println!("Adding skill");
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
	let new_skill = Skill {
		id: uuid::Uuid::new_v4(),
		label: skilldata.label.clone(),
		skillcategory_id: uuid::Uuid::new_v4(),
		skillscope_id: uuid::Uuid::new_v4(),
		updated_by: skilldata.email.clone(),
	};
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