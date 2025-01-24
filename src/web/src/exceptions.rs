use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ErrorResponse {
    BadRequest{message: String},
    Forbidden{message: String},
    InternalServer{message: String},
    Unauthorized{message: String}
}

impl ErrorResponse {
    pub fn status(&self) -> u16 {
        match self {
            ErrorResponse::BadRequest{message} => StatusCode::BAD_REQUEST.as_u16(),
            ErrorResponse::Forbidden{message} => StatusCode::FORBIDDEN.as_u16(),
            ErrorResponse::InternalServer{message} => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            ErrorResponse::Unauthorized{message} => StatusCode::UNAUTHORIZED.as_u16()
        }
    }

}

impl IntoResponse for ErrorResponse{
    fn into_response(self) -> Response {
        match self {
            ErrorResponse::BadRequest{message} => (StatusCode::BAD_REQUEST, message).into_response(),
            ErrorResponse::Forbidden{message} => (StatusCode::FORBIDDEN, message).into_response(),
            ErrorResponse::InternalServer{message} => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
            ErrorResponse::Unauthorized{message} => (StatusCode::UNAUTHORIZED, message).into_response()
        }
    }
}