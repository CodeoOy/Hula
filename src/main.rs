#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::{Session};
use actix_web::http::{header, StatusCode};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::connection::{MigrationConnection};
use diesel_migrations::*;

mod email_service;
mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

//embed_migrations!();

#[get("/")]
async fn home(session: Session) -> Result<HttpResponse> {
	//println!("{:?}", req);
	//println!("Lol");
	// session
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		println!("SESSION value: {}", count);
		counter = count + 1;
	}

	// set counter to session
	session.set("counter", counter)?;

	// response
	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

/// simple index handler
#[get("/app/*")]
async fn allviews(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	println!("{:?}", req);
	//println!("Lol");
	// session
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		println!("SESSION value: {}", count);
		counter = count + 1;
	}

	// set counter to session
	session.set("counter", counter)?;

	// response
	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}


pub fn establish_connection(name: &str) -> PgConnection {
    PgConnection::establish(&name)
        .expect(&format!("Error connecting to {}", name))
}

fn initialize_db(name: &str) {
	let connection = establish_connection(name);

	diesel_migrations::run_pending_migrations(&connection);

	// embedded_migrations::run(&connection);

    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    // embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	std::env::set_var("RUST_LOG", "simple-auth-server=debug,actix_web=info,actix_server=info");
	env_logger::init();
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let database_url2 = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	// create db connection pool
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool: models::users::Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
	let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

	initialize_db(&database_url2);

	// Start http server
	HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			// enable logger
			.wrap(middleware::Logger::default())
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
					.name("auth")
					.path("/")
					.domain(domain.as_str())
					.max_age(86400) // one day in seconds
					.secure(false), // this can only be true if you have https
			))
			.data(web::JsonConfig::default().limit(4096))
			// everything under '/api/' route
			.service(
				web::scope("/api")
					.service(
						web::resource("/invitation")
							.route(web::post().to(handlers::invitation_handler::post_invitation)),
					)
					.service(
						web::resource("/user/{user_id}")
							.route(web::get().to(handlers::users_handler::get_by_uuid))
							.route(web::put().to(handlers::users_handler::update_user))
							.route(web::delete().to(handlers::users_handler::delete_user)),
					)
					.service(
						web::resource("/userskill/{user_id}")
							.route(web::post().to(handlers::users_handler::add_skill))
							.route(web::put().to(handlers::users_handler::update_year)),
					)
					.service(web::resource("/skill").route(web::post().to(handlers::skills_handler::create_skill)))
					.service(
						web::resource("/skills/categories")
							.route(web::get().to(handlers::skills_handler::get_skill_categories)),
					)
					.service(
						web::resource("/skills/scopes").route(web::get().to(handlers::skills_handler::get_skillscopes)),
					)
					.service(
						web::resource("/skills/levels")
							.route(web::get().to(handlers::skills_handler::get_skill_levels)),
					)
					.service(web::resource("/skills").route(web::get().to(handlers::skills_handler::get_all_skills)))
					.service(
						web::resource("/skills/scope")
							.route(web::post().to(handlers::skills_handler::create_skill_scope)),
					)
					.service(
						web::resource("/skills/level")
							.route(web::post().to(handlers::skills_handler::create_skill_scope_level)),
					)
					.service(
						web::resource("/skills/category")
							.route(web::post().to(handlers::skills_handler::create_skill_category)),
					)
					.service(web::resource("/users").route(web::get().to(handlers::users_handler::get_all)))
					.service(web::resource("/project").route(web::post().to(handlers::projects_handler::get_by_pid)))
					.service(
						web::resource("/projects").route(web::get().to(handlers::projects_handler::get_all_projects)),
					)
					.service(
						web::resource("/matchedusers")
							.route(web::post().to(handlers::matches_handler::get_matches_by_params)),
					)
					.service(
						web::resource("/register/{invitation_id}")
							.route(web::post().to(handlers::register_handler::register_user)),
					)
					.service(web::resource("/matches").route(web::get().to(handlers::matches_handler::get_all_matches)))
					.service(
						web::resource("/auth")
							.route(web::post().to(handlers::auth_handler::login))
							.route(web::delete().to(handlers::auth_handler::logout))
							.route(web::get().to(handlers::auth_handler::get_me)),
					),
			)
			.service(fs::Files::new("/public", "public").show_files_listing())
			.service(home)
			.service(allviews)
			.service(web::resource("/").route(web::get().to(|req: HttpRequest| {
				println!("HTTP REQ:\n{:?}\n", req);
				HttpResponse::Found().header(header::LOCATION, "index.html").finish()
			})))
	})
	.bind("localhost:8086")?
	.run()
	.await
}
