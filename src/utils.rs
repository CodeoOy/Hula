use crate::errors::ServiceError;
use argon2::{self, Config};
use log::error;

lazy_static::lazy_static! {
	pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
}

const SALT: &'static [u8] = b"supersecuresalt";

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
	let config = Config {
		secret: SECRET_KEY.as_bytes(),
		..Default::default()
	};
	argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
		error!("Password encode error: err={:#?}", err);
		ServiceError::InternalServerError
	})
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
	argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(|err| {
		error!("Password decode error: err={:#?}", err);
		ServiceError::Unauthorized
	})
}
