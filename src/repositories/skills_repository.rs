use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::skills::{Pool, Skill};

pub fn query_all_skills(pool: &web::Data<Pool>) -> Result<Vec<Skill>, diesel::result::Error> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skills.load::<Skill>(conn)?;
	Ok(items)
}

pub fn query_create_skill(
	q_label: String,
	q_skillcategory_id: uuid::Uuid,
	q_skillscope_id: uuid::Uuid,
	pool: web::Data<Pool>,
	email: String,
) -> Result<Skill, crate::errors::ServiceError> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_skill = Skill {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		skillcategory_id: q_skillcategory_id,
		skillscope_id: q_skillscope_id,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(skills).values(&new_skill).get_result::<Skill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		return Ok(new_skill.into());
	}
	Err(ServiceError::Unauthorized)
}

pub fn query_delete_skill(uuid_data: String, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skills::dsl::id;
	use crate::schema::skills::dsl::*;

	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	diesel::delete(skills.filter(id.eq(uuid_query))).execute(conn)?;
	Ok(())
}
