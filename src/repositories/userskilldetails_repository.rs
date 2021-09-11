use crate::models::users::Pool;
use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};

use crate::models::users::User;
use crate::models::userskilldetails::UserSkillDetail;

pub fn find_by_users(users: &Vec<User>, pool: &web::Data<Pool>) -> Result<Vec<Vec<UserSkillDetail>>, Error> {
	use crate::schema::userskilldetails::dsl::skill_label;

	let conn: &PgConnection = &pool.get().unwrap();

	let skills = UserSkillDetail::belonging_to(users)
		.order(skill_label.asc())
		.load::<UserSkillDetail>(conn)?
		.grouped_by(&users);

	Ok(skills)
}
