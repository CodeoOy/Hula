/* pub mod projectskill; */

use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projectskills"]
pub struct ProjectSkill {
	pub id: uuid::Uuid,
	pub projectid: uuid::Uuid,
	pub skillid: uuid::Uuid,
	pub skillscopelevelid: Option<uuid::Uuid>,
	pub minyears: Option<f32>,
	pub maxyears: Option<f32>,
	pub countofusers: i32,
	pub begin_time: chrono::NaiveDateTime,
	pub end_time: chrono::NaiveDateTime,
	pub percentage: i32,
	pub inserted_by: String,
	pub inserted_at: chrono::NaiveDateTime,
	pub updated_by: String,
	pub updated_at: chrono::NaiveDateTime,
	pub updated_count: i32,
}
	
/*        begin_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        percentage -> Nullable<Int2>,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
*/
	

/*
skillid -> Uuid,
skillscopelevelid -> Nullable<Uuid>,
minyears -> Nullable<Numeric>,
maxyears -> Nullable<Numeric>,
countofusers -> Numeric,
begin_time -> Timestamp,
end_time -> Nullable<Timestamp>,
percentage -> Nullable<Int2>,
inserted_by -> Varchar,
inserted_at -> Timestamp,
updated_by -> Varchar,
updated_at -> Timestamp,
updated_count -> Int2,
}
*/