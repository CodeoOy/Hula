//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// simple index handler
#[get("/")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);
	println!("Lol");
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
	println!("Lol");
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
#[get("/omnom")]
async fn omnom(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);
	println!("Om noms");
    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }
    // set counter to session
    session.set("counter", counter)?;

	let treefloof = "Cheeseboi!".to_string();

	Ok(HttpResponse::Ok()
        .json(treefloof))
}

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
		println!("{:?}", user);
        Ok(HttpResponse::Ok().json(user))
    } else {
		println!("No user");
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8082";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
			.service(welcome)
			.service(allviews)
			.service(omnom)
			// static files
            .service(fs::Files::new("/public", "public").show_files_listing())
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "public/index.html")
                    .finish()
            })))
    })
    .bind(&bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn user_routes() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<MysqlConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .service(get_user)
                .service(add_user),
        )
        .await;

        // Insert a user
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&models::NewUser {
                name: "Test user".to_owned(),
            })
            .to_request();

        let resp: models::User = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.name, "Test user");

        // Get a user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", resp.id))
            .to_request();

        let resp: models::User = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.name, "Test user");

        // Delete new user from table
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(resp.id)))
            .execute(&pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");
    }
}