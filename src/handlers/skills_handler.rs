use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::skills::{Skill};

fn create_skill(invdata: InvitationData, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let invitation = dbg!(query(
		invdata.email,
		invdata.password_plain,
		invdata.first_name,
		invdata.last_name,
		pool
	)?);
	send_invitation(&invitation)
}

/// Diesel query
fn query(
	eml: String,
	psw: String,
	first_name: String,
	last_name: String,
	pool: web::Data<Pool>,
) -> Result<Invitation, crate::errors::ServiceError> {
	use crate::schema::invitations::dsl::invitations;

	let password: String = hash_password(&psw)?;
	let new_invitation = Invitation::from_details(eml, password, first_name, last_name);
	let conn: &PgConnection = &pool.get().unwrap();

	let inserted_invitation = diesel::insert_into(invitations)
		.values(&new_invitation)
		.get_result(conn)?;

	Ok(inserted_invitation)
}

pub async fn add_skill(
	uuid_data: web::Path<String>,
	payload: web::Json<UserSkill>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	println!("Adding skill");
	let res = web::block(move || query_add_skill(uuid_data.into_inner(), payload, pool)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_add_skill(
	uuid_data: String,
	skill_data: web::Json<UserSkill>,
	pool: web::Data<Pool>,
) -> Result<UserSkill, crate::errors::ServiceError> {
	use crate::schema::userskills::dsl::userskills;
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data)?;
	let new_user_skill = UserSkill {
		id: skill_data.id,
		user_id: uuid_query,
		skill_id: skill_data.skill_id,
		skillscopelevel_id: skill_data.skillscopelevel_id,
		years: skill_data.years,
		updated_by: String::from("Kylpynalle"), // LoggedUser here
	};
	let rows_inserted = diesel::insert_into(userskills)
		.values(&new_user_skill)
		.get_result::<UserSkill>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nSkill added successfully.\n");
		return Ok(new_user_skill.into());
	}
	Err(ServiceError::Unauthorized)
}