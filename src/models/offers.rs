use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug, Insertable)]
#[table_name = "offers"]
pub struct Offer {
	pub id: uuid::Uuid,
	pub user_id: uuid::Uuid,
	pub project_id: uuid::Uuid,
	pub sold: bool,
	pub comments: Option<String>,
	pub updated_by: String,
}
