use actix_web::web;
use diesel::{prelude::*, PgConnection};

use crate::errors::ServiceError;
use crate::models::matchcandidates::{Pool, MatchCandidate};

pub fn query(pool: &web::Data<Pool>) -> Result<Vec<MatchCandidate>, ServiceError> {
	use crate::schema::matchcandidates::dsl::matchcandidates;
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = matchcandidates.load::<MatchCandidate>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.available == true);

	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub fn query_by_params(
	q_project_name: String,
	pool: &web::Data<Pool>,
) -> Result<Vec<MatchCandidate>, crate::errors::ServiceError> {
	use crate::schema::matchcandidates::dsl::matchcandidates;
	let conn: &PgConnection = &pool.get().unwrap();
	let mut items = matchcandidates.load::<MatchCandidate>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.available == true);
	items.retain(|x| x.projectname == q_project_name);

	if items.is_empty() == false {
		return Ok(items);
	}
	Err(ServiceError::Empty)
}
