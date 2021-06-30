use actix_web::web;
use diesel::{prelude::*, PgConnection};
use diesel::result::Error;

use crate::models::skills::{Pool, SkillScope};

pub fn query_skill_scopes(pool: &web::Data<Pool>) -> Result<Vec<SkillScope>, Error> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillscopes.load::<SkillScope>(conn)?;
	Ok(items)
}

pub fn create_skill_scope(
	q_label: String,
	q_email: String,
	pool: &web::Data<Pool>,
 ) -> Result<SkillScope, Error> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_scope = SkillScope {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		updated_by: q_email,
	};

	diesel::insert_into(skillscopes)
		.values(&new_scope)
		.execute(conn)?;

	Ok(new_scope.into())
}

pub fn update_skill_scope(
	uuid_data: uuid::Uuid,
	q_label: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Option<SkillScope>, Error> {
	use crate::schema::skillscopes::dsl::{skillscopes, *};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut scope = diesel::update(skillscopes)
		.filter(id.eq(uuid_data))
		.set((
			label.eq(q_label),
			updated_by.eq(q_email.clone()),
		))
		.load::<SkillScope>(conn)?;

	if let Some(scope_res) = scope.pop() {
		return Ok(scope_res.into());
	}

	Ok(None)
}

