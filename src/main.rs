#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod auth_handler;
mod email_service;
mod errors;
mod invitation_handler;
mod models;
mod register_handler;
mod schema;
mod utils;
mod oneuser_handler;
mod users_handler;
mod oneproject_handler;
mod projects_handler;
#[path = "handlers/matches_handler.rs"]
mod matches_handler;

mod models2;

#[get("/")]
async fn home(session: Session, req: HttpRequest) -> Result<HttpResponse> {
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	std::env::set_var(
		"RUST_LOG",
		"simple-auth-server=debug,actix_web=info,actix_server=info",
	);
	env_logger::init();
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	// create db connection pool
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool: models::Pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.");
	let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

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
							.route(web::post().to(invitation_handler::post_invitation)),
					)
					.service(
						web::resource("/user/{user_id}")
							.route(web::get().to(oneuser_handler::get_by_uuid))
							.route(web::put().to(oneuser_handler::update_user)),
					)
					.service(
						web::resource("/users")
							.route(web::get().to(users_handler::get_all)),
					)
					.service(
						web::resource("/project")
							.route(web::post().to(oneproject_handler::get_by_pid)),
					)
					.service(
						web::resource("/projects")
							.route(web::get().to(projects_handler::get_all_projects)),
					)
					.service(
						web::resource("/register/{invitation_id}")
							.route(web::post().to(register_handler::register_user)),
					)
					.service(
						web::resource("/matches")
							.route(web::get().to(matches_handler::get_all_matches)),
					)
					.service(
						web::resource("/auth")
							.route(web::post().to(auth_handler::login))
							.route(web::delete().to(auth_handler::logout))
							.route(web::get().to(auth_handler::get_me)),
					),
			)
			.service(fs::Files::new("/public", "public").show_files_listing())
			.service(home)
			.service(allviews)
			.service(web::resource("/").route(web::get().to(|req: HttpRequest| {
				println!("HTTP REQ:\n{:?}\n", req);
				HttpResponse::Found()
					.header(header::LOCATION, "index.html")
					.finish()
			})))
	})
	.bind("localhost:8086")?
	.run()
	.await
}
