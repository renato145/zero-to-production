use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

// Return an opaque 500 while preserving the error root cause for logging.
pub fn e500<T>(e: T) -> actix_web::error::InternalError<T> {
    actix_web::error::InternalError::from_response(e, HttpResponse::InternalServerError().finish())
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn e400<T: std::fmt::Display>(e: T) -> actix_web::error::InternalError<T> {
    actix_web::error::InternalError::new(e, StatusCode::BAD_REQUEST)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
