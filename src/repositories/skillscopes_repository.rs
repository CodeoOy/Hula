use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

use crate::models::skills::{Pool, SkillScope};

pub fn query_skill_scopes(pool: &web::Data<Pool>) -> Result<Vec<SkillScope>, Error> {
	use crate::schema::skillscopes::dsl::{label, skillscopes};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = skillscopes.order(label.asc()).load::<SkillScope>(conn)?;

	Ok(items)
}

pub fn create_skill_scope(q_label: String, q_email: String, pool: &web::Data<Pool>) -> Result<SkillScope, Error> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_scope = SkillScope {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		updated_by: q_email,
	};

	let scope = diesel::insert_into(skillscopes)
		.values(&new_scope)
		.get_result::<SkillScope>(conn)?;

	Ok(scope)
}

pub fn update_skill_scope(
	uuid_data: uuid::Uuid,
	q_label: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<SkillScope, Error> {
	use crate::schema::skillscopes::dsl::{skillscopes, *};
	let conn: &PgConnection = &pool.get().unwrap();

	let scope = diesel::update(skillscopes)
		.filter(id.eq(uuid_data))
		.set((label.eq(q_label), updated_by.eq(q_email.clone())))
		.get_result::<SkillScope>(conn)?;

	Ok(scope)
}

pub fn delete_skill_scope(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skillscopes::dsl::{id, skillscopes};

	let deleted = diesel::delete(skillscopes.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
