use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;

use crate::models::skills::{Pool, SkillCategory};

pub fn query_skill_categories(pool: web::Data<Pool>) -> Result<Vec<SkillCategory>, Error> {
	use crate::schema::skillcategories::dsl::skillcategories;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillcategories.load::<SkillCategory>(conn)?;
	Ok(items)
}

pub fn create_skill_category(
	q_label: String,
	q_parent_id: Option<uuid::Uuid>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<SkillCategory, Error> {
	use crate::schema::skillcategories::dsl::skillcategories;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_skill_category = SkillCategory {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		parent_id: q_parent_id,
		updated_by: q_email,
	};
	
	diesel::insert_into(skillcategories)
		.values(&new_skill_category)
		.execute(conn)?;

	Ok(new_skill_category.into())
}

pub fn delete_skill_category(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<usize, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skillcategories::dsl::{skillcategories, id};

	let deleted = diesel::delete(skillcategories.filter(id.eq(uuid_data))).execute(conn)?;
	Ok(deleted)
}

