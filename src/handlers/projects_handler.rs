use actix_web::{error::BlockingError, web, HttpResponse, HttpRequest};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};

use crate::handlers::auth_handler::LoggedUser;
use crate::{errors::ServiceError, handlers::auth_handler, models::users::SlimUser};
use crate::models::projects::{Pool, Project};

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectData {
	pub name: String,
}

pub async fn get_all_projects(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting all projects");
	let res = web::block(move || query_all(pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_all(pool: web::Data<Pool>) -> Result<Vec<Project>, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();
	let items = projects.load::<Project>(conn)?;
	if items.is_empty() == false {
		println!("\nGot all projects maybe.\n");
		return Ok(items);
	}
	Err(ServiceError::Empty)
}

pub async fn create_project(
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
	println!("Creating a project");
	let res = web::block(move || query_create_project(projectdata, pool, logged_user.email)).await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		}
    }
}

fn query_create_project(
	projectdata: web::Json<ProjectData>,
	pool: web::Data<Pool>,
	email: String
) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::projects;
	let conn: &PgConnection = &pool.get().unwrap();
	println!("Connected to db");
	let new_project = Project {
		id: uuid::Uuid::new_v4(),
		available: true,
		name: projectdata.name.clone(),
		updated_by: email
	};
	println!("Inserting data");
	let rows_inserted = diesel::insert_into(projects)
		.values(&new_project)
		.get_result::<Project>(conn);
	println!("{:?}", rows_inserted);
	if rows_inserted.is_ok() {
		println!("\nProject added successfully.\n");
		return Ok(new_project.into());
	}
	Err(ServiceError::Unauthorized)
}

pub async fn get_by_pid(uuid_data: web::Json<QueryData>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
	// run diesel blocking code
	println!("\nGetting project by uuid");
	let res = web::block(move || query_one(uuid_data.into_inner(), pool)).await;

	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_one(uuid_data: QueryData, pool: web::Data<Pool>) -> Result<Project, crate::errors::ServiceError> {
	use crate::schema::projects::dsl::{id, projects};
	let conn: &PgConnection = &pool.get().unwrap();
	let uuid_query = uuid::Uuid::parse_str(&uuid_data.id)?;
	let mut items = projects.filter(id.eq(uuid_query)).load::<Project>(conn)?;
	if let Some(project_res) = items.pop() {
		println!("\nQuery successful.\n");
		return Ok(project_res.into());
	}
	Err(ServiceError::Unauthorized)
}
