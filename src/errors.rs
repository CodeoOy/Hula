use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as ParseError;
use log::error;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
#[display(fmt = "{} {} {}", table_name, field_name, description)]
pub struct ForbiddenReference {
	pub table_name: String,
	pub field_name: String,
	pub description: String,
}

#[derive(Debug, Display, Serialize)]
pub enum ForbiddenType {
	#[display(fmt = "Admin required")]
	AdminRequired,

	#[display(fmt = "Unique violated at: {}", _0)]
	UniqueViolation(ForbiddenReference),

	#[display(fmt = "Foreign key violated: {}", _0)]
	ForeignKeyViolation(ForbiddenReference),
}


#[derive(Debug, Display)]
pub enum ServiceError {
	#[display(fmt = "Internal Server Error")]
	InternalServerError,

	#[display(fmt = "BadRequest: {}", _0)]
	BadRequest(String),

	#[display(fmt = "Forbidden: {}", _0)]
	Forbidden(ForbiddenType),

	#[display(fmt = "Unauthorized")]
	Unauthorized,

	#[display(fmt = "Empty result")]
	Empty,

	#[display(fmt = "Gone")]
	Gone,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
	fn error_response(&self) -> HttpResponse {
		match self {
			ServiceError::InternalServerError => {
				HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
			}
			ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
			ServiceError::Unauthorized => HttpResponse::Unauthorized().finish(),
			ServiceError::Empty => HttpResponse::NoContent().finish(),
			ServiceError::Gone => HttpResponse::Gone().finish(),
			ServiceError::Forbidden(ref forbidden_type) => HttpResponse::Forbidden().json(forbidden_type),
		}
	}
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
	fn from(_: ParseError) -> ServiceError {
		ServiceError::BadRequest("Invalid UUID".into())
	}
}

impl From<DBError> for ServiceError {
	fn from(error: DBError) -> ServiceError {

		// Right now we just care about UniqueViolation from diesel
		// But this would be helpful to easily map errors as our app grows
		match error {
			DBError::DatabaseError(kind, info) => {
				match kind {
					DatabaseErrorKind::UniqueViolation => {
						let description = format!("{} {} {}", &info.message(), &info.details().unwrap_or_default(), &info.hint().unwrap_or_default());
						let field_name = info.constraint_name().unwrap_or_default().split('_').last().unwrap_or_default();
						return ServiceError::Forbidden(ForbiddenType::UniqueViolation(
							ForbiddenReference {
								table_name: String::from(info.table_name().unwrap_or_default()), 
								field_name: String::from(field_name), 
								description: description, 
							}));
					}

					DatabaseErrorKind::ForeignKeyViolation => {
						let description = format!("{} {} {}", &info.message(), &info.details().unwrap_or_default(), &info.hint().unwrap_or_default());
						let field_name = info.constraint_name().unwrap_or_default().split('_').last().unwrap_or_default();
						return ServiceError::Forbidden(ForbiddenType::ForeignKeyViolation(
							ForbiddenReference {
								table_name: String::from(info.table_name().unwrap_or_default()), 
								field_name: String::from(field_name), 
								description: description, 
							}));
					}

					_ => {
						error!("Database query (DIESEL) failed: {:?} {:#?}", &kind, &info);
					}
				}

				ServiceError::InternalServerError
			} 
			error => {
				error!("Database query (DIESEL) failed: {:#?}", error);
				ServiceError::InternalServerError
			}
		}
	}
}
