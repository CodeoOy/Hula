use actix_web::web;
use diesel::{prelude::*, PgConnection};

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
) -> Result<Skill, diesel::result::Error> {
	use crate::schema::skills::dsl::skills;
	let conn: &PgConnection = &pool.get().unwrap();
	let new_skill = Skill {
		id: uuid::Uuid::new_v4(),
		label: q_label,
		skillcategory_id: q_skillcategory_id,
		skillscope_id: q_skillscope_id,
		updated_by: email,
	};
	let _rows_inserted = diesel::insert_into(skills).values(&new_skill).get_result::<Skill>(conn)?;
	Ok(new_skill.into())
}

pub fn query_delete_skill(uuid_data: uuid::Uuid, pool: web::Data<Pool>) -> Result<(), diesel::result::Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skills::dsl::id;
	use crate::schema::skills::dsl::*;

	diesel::delete(skills.filter(id.eq(uuid_data))).execute(conn)?;
	Ok(())
}
