use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::skills::{Pool, SkillScope};

pub fn query_skillscopes(pool: web::Data<Pool>) -> Result<Vec<SkillScope>, crate::errors::ServiceError> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillscopes.load::<SkillScope>(conn)?;
	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn query_create_skill_scope(
	q_label: String,
	pool: web::Data<Pool>,
	email: String,
) -> Result<SkillScope, crate::errors::ServiceError> {
	use crate::schema::skillscopes::dsl::skillscopes;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_scope = SkillScope {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		updated_by: email,
	};
	let rows_inserted = diesel::insert_into(skillscopes)
		.values(&new_scope)
		.get_result::<SkillScope>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		return Ok(new_scope.into());
	}
	Err(ServiceError::Unauthorized)
}

