use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

use crate::models::skills::{Pool, SkillCategory};

pub fn query_skill_categories(pool: web::Data<Pool>) -> Result<Vec<SkillCategory>, Error> {
	use crate::schema::skillcategories::dsl::{skillcategories, *};
	let conn: &PgConnection = &pool.get().unwrap();
	
	let items = skillcategories
		.order(label.asc())
		.load::<SkillCategory>(conn)?;

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

	let skillcategory = diesel::insert_into(skillcategories)
		.values(&new_skill_category)
		.get_result::<SkillCategory>(conn)?;

	Ok(skillcategory)
}

pub fn update_skill_categories(
	uuid_data: uuid::Uuid,
	q_label: String,
	q_parent_id: Option<uuid::Uuid>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<SkillCategory, Error> {
	use crate::schema::skillcategories::dsl::{skillcategories, *};
	let conn: &PgConnection = &pool.get().unwrap();

	let skillcategory = diesel::update(skillcategories)
		.filter(id.eq(uuid_data))
		.set((
			label.eq(q_label),
			parent_id.eq(q_parent_id),
			updated_by.eq(q_email.clone()),
		))
		.get_result::<SkillCategory>(conn)?;

	Ok(skillcategory)
}

pub fn delete_skill_category(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skillcategories::dsl::{id, skillcategories};

	let deleted = diesel::delete(skillcategories.filter(id.eq(uuid_data))).execute(conn)?;
	
	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
