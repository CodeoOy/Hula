use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::Serialize;

use crate::errors::ServiceError;
use crate::models::projectmatches::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;

#[derive(Serialize, Debug)]
pub struct ProjectDTO {
	pub id: uuid::Uuid,
	pub name: String,
	pub is_hidden: bool,
	pub skills: Vec<SkillDTO>,
	pub matches: Vec<MatchDTO>,
}

#[derive(Serialize, Debug)]
pub struct SkillDTO {
	pub skill_label: String,
	pub skill_mandatory: bool,
}

#[derive(Serialize, Debug)]
pub struct MatchDTO {
	pub user_id: uuid::Uuid,
	pub first_name: String,
	pub last_name: String,
	pub has_mandatory: bool,
	pub is_all_skills: bool,
	pub is_available: bool,
	pub tier: i32,
}

pub async fn get_project_matchdata(
	pid: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all matches: logged_user={:#?}", &logged_user);

	let id = uuid::Uuid::parse_str(&pid.into_inner())?;

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let project = projects_repository::query_one(id, &pool)?;
	let res = web::block(move || projectmatches_repository::find_by_project(&project, &pool)).await;

	match res {
		Ok(matches) => {
			if matches.is_empty() == false {
				println!("\n\nMatches: {:?}\n", matches);
				return Ok(HttpResponse::Ok().json(&matches));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
