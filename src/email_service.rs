use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, ResetPasswordRequest};
use log::{error, trace};
use sparkpost::transmission::{EmailAddress, Message, Options, Recipient, Transmission, TransmissionResponse};
use url::Url;

lazy_static::lazy_static! {
static ref API_KEY: String = std::env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set");
}

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
	let tm = Transmission::new(API_KEY.as_str());
	let sending_email = std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
	let public_url = std::env::var("PUBLIC_URL").unwrap_or_else(|_| "localhost:8086".to_string());

	let mut email = Message::new(EmailAddress::new(sending_email, "Hula"));

	let options = Options {
		open_tracking: false,
		click_tracking: false,
		transactional: true,
		sandbox: false,
		inline_css: false,
		start_time: None,
	};

	// recipient from the invitation email
	let recipient: Recipient = invitation.email.as_str().into();

	let base = format!("{}/app/confirm", public_url);

	let url = Url::parse_with_params(
		&base,
		&[
			("id", invitation.id.to_string()),
			("email", invitation.email.clone()),
			(
				"password",
				invitation.password_plain.clone().unwrap_or_else(|| "".to_string()),
			),
		],
	)
	.expect("failed to construct URL. Check your PUBLIC_URL parameter.");

	let email_body = format!(
		"Please click on the link below to complete registration. <br/>
			<a href=\"{}\">
			Click here</a> <br>
			your Invitation expires on <strong>{}</strong>",
		url.as_str(),
		invitation.expires_at.format("%I:%M %p %A, %-d %B, %C%y").to_string()
	);

	// complete the email message with details
	email
		.add_recipient(recipient)
		.options(options)
		.subject("You have been invited to join Hula ERP.")
		.html(email_body);

	let result = tm.send(&email);

	// Note that we only print out the error response from email api
	match result {
		Ok(res) => match res {
			TransmissionResponse::ApiResponse(api_res) => {
				trace!("API Response: \n {:#?}", api_res);
				Ok(())
			}
			TransmissionResponse::ApiError(errors) => {
				error!("Response Errors: \n {:#?}", &errors);
				Err(ServiceError::InternalServerError)
			}
		},
		Err(error) => {
			error!("Send Email Error: \n {:#?}", error);
			Err(ServiceError::InternalServerError)
		}
	}
}

pub fn send_reset_request(invitation: &ResetPasswordRequest) -> Result<(), ServiceError> {
	let tm = Transmission::new(API_KEY.as_str());
	let sending_email = std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
	let public_url = std::env::var("PUBLIC_URL").unwrap_or_else(|_| "localhost:8086".to_string());

	let mut email = Message::new(EmailAddress::new(sending_email, "Hula"));

	let options = Options {
		open_tracking: false,
		click_tracking: false,
		transactional: true,
		sandbox: false,
		inline_css: false,
		start_time: None,
	};

	// recipient from the invitation email
	let recipient: Recipient = invitation.email.as_str().into();

	let base = format!("{}/app/confirm", public_url);

	let url = Url::parse_with_params(
		&base,
		&[("id", invitation.id.to_string()), ("email", invitation.email.clone())],
	)
	.expect("failed to construct URL. Check your PUBLIC_URL parameter.");

	let email_body = format!(
		"Please click on the link below to complete registration. <br/>
			<a href=\"{}\">
			Click here</a> <br>
			your Invitation expires on <strong>{}</strong>",
		url.as_str(),
		invitation.expires_at.format("%I:%M %p %A, %-d %B, %C%y").to_string()
	);

	// complete the email message with details
	email
		.add_recipient(recipient)
		.options(options)
		.subject("You have been invited to join Hula ERP.")
		.html(email_body);

	let result = tm.send(&email);

	// Note that we only print out the error response from email api
	match result {
		Ok(res) => match res {
			TransmissionResponse::ApiResponse(api_res) => {
				trace!("API Response: \n {:#?}", api_res);
				Ok(())
			}
			TransmissionResponse::ApiError(errors) => {
				error!("Response Errors: \n {:#?}", &errors);
				Err(ServiceError::InternalServerError)
			}
		},
		Err(error) => {
			error!("Send Email Error: \n {:#?}", error);
			Err(ServiceError::InternalServerError)
		}
	}
}
