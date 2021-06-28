use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::skills::{Pool, SkillScopeLevel};

pub fn query_skill_levels(pool: web::Data<Pool>) -> Result<Vec<SkillScopeLevel>, crate::errors::ServiceError> {
	use crate::schema::skillscopelevels::dsl::skillscopelevels;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillscopelevels.load::<SkillScopeLevel>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn query_create_skill_scope_level(
	q_label: String,
	q_percentage: Option<i32>,
	q_skillscope_id: uuid::Uuid,
	pool: web::Data<Pool>,
	email: String,
) -> Result<SkillScopeLevel, crate::errors::ServiceError> {
	use crate::schema::skillscopelevels::dsl::{index, skillscope_id, skillscopelevels};
	let conn: &PgConnection = &pool.get().unwrap();
	let mut currentindex = 0;
	let latestlevel = skillscopelevels
		.filter(skillscope_id.eq(q_skillscope_id))
		.order_by(index.desc())
		.limit(1)
		.load::<SkillScopeLevel>(conn)?;
	if latestlevel.len() > 0 {
		currentindex = latestlevel[0].index;
	}
	let new_scope_level = SkillScopeLevel {
		id: uuid::Uuid::new_v4(),
		label: q_label, // TODO: This is unique, sanitize field or handle error more gracefully
		skillscope_id: q_skillscope_id,
		index: currentindex + 1,
		percentage: q_percentage,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(skillscopelevels)
		.values(&new_scope_level)
		.get_result::<SkillScopeLevel>(conn);
	if rows_inserted.is_ok() {
		return Ok(new_scope_level.into());
	}
	Err(ServiceError::Unauthorized)
}
