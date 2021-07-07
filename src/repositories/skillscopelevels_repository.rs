use actix_web::web;
use derive_more::Display;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};
use serde::Serialize;

use crate::models::skills::{Pool, SkillScopeLevel};

#[derive(Debug, Display, Serialize)]
pub enum ScopeLevelSwapDirection {
	Better,
	Worse,
}

pub fn query_skill_levels(pool: &web::Data<Pool>) -> Result<Vec<SkillScopeLevel>, Error> {
	use crate::schema::skillscopelevels::dsl::skillscopelevels;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = skillscopelevels.load::<SkillScopeLevel>(conn)?;
	Ok(items)
}

pub fn create_skill_scope_level(
	q_label: String,
	q_percentage: Option<i32>,
	q_skillscope_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<SkillScopeLevel, Error> {
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
		updated_by: q_email,
	};

	diesel::insert_into(skillscopelevels)
		.values(&new_scope_level)
		.execute(conn)?;

	Ok(new_scope_level.into())
}

pub fn update_skill_scope_level(
	uuid_data: uuid::Uuid,
	q_label: String,
	q_percentage: Option<i32>,
	q_email: String,
	q_swap_direction: Option<ScopeLevelSwapDirection>,
	pool: &web::Data<Pool>,
) -> Result<Option<SkillScopeLevel>, Error> {
	let conn: &PgConnection = &pool.get().unwrap();

	let mut scopelevel: Vec<SkillScopeLevel> = vec![];
	conn.transaction::<_, Error, _>(|| {
		use crate::schema::skillscopelevels::dsl::{skillscopelevels, *};

		if let Some(direction) = q_swap_direction {
			let current = skillscopelevels
				.filter(id.eq(uuid_data))
				.get_result::<SkillScopeLevel>(conn)?;

			let mut levels = skillscopelevels
				.filter(skillscope_id.eq(current.skillscope_id))
				.order(index.asc())
				.load::<SkillScopeLevel>(conn)?;

			let other = match direction {
				ScopeLevelSwapDirection::Better => {
					levels.retain(|x| x.index > current.index);
					levels.first()
				}
				ScopeLevelSwapDirection::Worse => {
					levels.retain(|x| x.index < current.index);
					levels.last()
				}
			};

			println!("other = {:#?}", other);

			if let Some(other) = other {
				diesel::update(skillscopelevels)
					.filter(id.eq(other.id))
					.set((index.eq(0), updated_by.eq(q_email.clone())))
					.execute(conn)?;
						
				diesel::update(skillscopelevels)
					.filter(id.eq(uuid_data))
					.set((index.eq(other.index), updated_by.eq(q_email.clone())))
					.execute(conn)?;

				diesel::update(skillscopelevels)
					.filter(id.eq(other.id))
					.set((index.eq(current.index), updated_by.eq(q_email.clone())))
					.execute(conn)?;
			}
		}

		scopelevel = diesel::update(skillscopelevels)
			.filter(id.eq(uuid_data))
			.set((
				label.eq(q_label),
				percentage.eq(q_percentage),
				updated_by.eq(q_email.clone()),
		))
		.load::<SkillScopeLevel>(conn)?;

		Ok(())
	})?;

	if let Some(scopelevel_res) = scopelevel.pop() {
		return Ok(scopelevel_res.into());
	}

	Ok(None)
}

pub fn delete_skill_scope_level(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<usize, Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::skillscopelevels::dsl::{id, skillscopelevels};

	let deleted = diesel::delete(skillscopelevels.filter(id.eq(uuid_data))).execute(conn)?;
	Ok(deleted)
}
