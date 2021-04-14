use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::email_service::send_invitation;
use crate::errors::ServiceError;
use crate::models::{Invitation, Pool};
use crate::utils::hash_password;

#[derive(Deserialize, Debug)]
pub struct InvitationData {
    pub email: String,
    pub password_plain: String,
}

pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // run diesel blocking code
    let res = web::block(move || create_invitation(invitation_data.into_inner(), pool)).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}
fn create_invitation(
    invdata: InvitationData,
    pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
    let invitation = dbg!(query(invdata.email, invdata.password_plain, pool)?);
    send_invitation(&invitation)
}

/// Diesel query
fn query(
    eml: String,
    psw: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let password: String = hash_password(&psw)?;
    let new_invitation = Invitation::from_details(eml, password);
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
