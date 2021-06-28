use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::skills::{Pool, SkillCategory};

pub fn query_skill_categories(pool: web::Data<Pool>) -> Result<Vec<SkillCategory>, crate::errors::ServiceError> {
	use crate::schema::skillcategories::dsl::skillcategories;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillcategories.load::<SkillCategory>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn query_create_skill_category(
	q_label: String,
	q_parent_id: Option<uuid::Uuid>,
	pool: web::Data<Pool>,
	email: String,
) -> Result<SkillCategory, crate::errors::ServiceError> {
	use crate::schema::skillcategories::dsl::skillcategories;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_skill_category = SkillCategory {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		parent_id: q_parent_id,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(skillcategories)
		.values(&new_skill_category)
		.get_result::<SkillCategory>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		return Ok(new_skill_category.into());
	}
	Err(ServiceError::Unauthorized)
}

