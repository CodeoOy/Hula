use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};

use crate::models::matchcandidates::{MatchCandidate, Pool};

pub fn query(pool: &web::Data<Pool>) -> Result<Vec<MatchCandidate>, Error> {
	use crate::schema::matchcandidates::dsl::{matchcandidates, projectneedskillid, userskillid};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = matchcandidates
		.order((projectneedskillid.asc(), userskillid.asc()))
		.load::<MatchCandidate>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.is_hidden == false);

	Ok(items)
}

pub fn query_by_params(q_project_name: String, pool: &web::Data<Pool>) -> Result<Vec<MatchCandidate>, Error> {
	use crate::schema::matchcandidates::dsl::{matchcandidates, projectneedskillid, userskillid};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = matchcandidates
		.order((projectneedskillid.asc(), userskillid.asc()))
		.load::<MatchCandidate>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.is_hidden == false);
	items.retain(|x| x.projectname == q_project_name);

	Ok(items)
}
