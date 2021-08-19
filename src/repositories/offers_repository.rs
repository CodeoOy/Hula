use actix_web::web;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use diesel::{prelude::*, PgConnection};

use crate::models::offers::{Offer, Pool};

pub fn query_get_all_offers(pool: &web::Data<Pool>) -> Result<Vec<Offer>, Error> {
	use crate::schema::offers::dsl::{id, offers, project_id};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = offers.order((id.asc(), project_id.asc())).load::<Offer>(conn)?;

	Ok(items)
}

pub fn query_add_offer(
	q_user_id: uuid::Uuid,
	q_project_id: uuid::Uuid,
	q_comments: Option<String>,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Offer, Error> {
	use crate::schema::offers::dsl::offers;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_offer = Offer {
		id: uuid::Uuid::new_v4(),
		user_id: q_user_id,
		project_id: q_project_id,
		sold: false,
		comments: q_comments,
		updated_by: q_email,
	};

	let offer = diesel::insert_into(offers)
		.values(&new_offer)
		.get_result::<Offer>(conn)?;

	Ok(offer)
}

pub fn query_delete_offer(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::offers::dsl::{id, offers};

	let deleted = diesel::delete(offers.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
