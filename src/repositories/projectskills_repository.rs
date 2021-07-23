use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};

use crate::models::projectskills::{Pool, Projectskills};

pub fn query_by_params(q_project_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<Vec<Projectskills>, Error> {
	use crate::schema::projectskills::dsl::*;
	use crate::schema::projectskills::dsl::{pn_id, project_id, projectskills};
	let conn: &PgConnection = &pool.get().unwrap();

	let mut items = projectskills
		.order((project_id.asc(), pn_id.asc()))
		.load::<Projectskills>(conn)?;

	items.retain(|x| x.project_id == q_project_id);

	Ok(items)
}
