use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};

use crate::models::projectmatches::{Pool, Projectmatches};

pub fn query_by_params(q_project_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<Vec<Projectmatches>, Error> {
	use crate::schema::projectmatches::dsl::{pn_id, project_id, projectmatches};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = projectmatches
		.order((project_id.asc(), pn_id.asc()))
		.load::<Projectmatches>(conn)?;

	items.retain(|x| x.required_index <= x.user_index);
	items.retain(|x| x.required_minyears <= x.user_years);
	items.retain(|x| x.required_maxyears >= x.user_years);
	items.retain(|x| x.user_is_hidden == false);
	items.retain(|x| x.project_id == q_project_id);

	Ok(items)
}
