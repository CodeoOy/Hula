use crate::errors::ServiceError;
use crate::models::invitations::Invitation;
use sparkpost::transmission::{EmailAddress, Message, Options, Recipient, Transmission, TransmissionResponse};

lazy_static::lazy_static! {
static ref API_KEY: String = std::env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set");
}

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
	let tm = Transmission::new(API_KEY.as_str());
	let sending_email = std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
	let public_url = std::env::var("PUBLIC_URL").unwrap_or_else(|_| "localhost:8086".to_string());

	// new email message with sender name and email
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

	let email_body = format!(
		"Please click on the link below to complete registration. <br/>
         <a href=\"{}/app/confirm?id={}&email={}&password={}\">
         Click here</a> <br>
         your Invitation expires on <strong>{}</strong>",
		public_url,
		invitation.id,
		invitation.email,
		invitation.password_plain,
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
				println!("API Response: \n {:#?}", api_res);
				Ok(())
			}
			TransmissionResponse::ApiError(errors) => {
				println!("Response Errors: \n {:#?}", &errors);
				Err(ServiceError::InternalServerError)
			}
		},
		Err(error) => {
			println!("Send Email Error: \n {:#?}", error);
			Err(ServiceError::InternalServerError)
		}
	}
}
