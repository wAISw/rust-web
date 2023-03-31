#![allow(clippy::enum_variant_names)]

pub(crate) type RestApiResult<T, E = RestApiError> = std::result::Result<T, E>;

#[derive(Debug)]
pub(crate) enum RestApiError {
    RestApiServiceError {
        source: String,
    },
}

impl RestApiError {
    pub fn rest_response_message(&self) -> &'static str {
        match self {
            Self::RestApiServiceError { source: _ } => {
                rocket::error_!("{self:?}");
                r#"{"error": "Internal Server Error"}"#
            }
        }
    }
}

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for RestApiError {
    fn respond_to(self, _: &'r rocket::request::Request<'_>) -> rocket::response::Result<'o> {
        let msg = self.rest_response_message();
        rocket::response::Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::InternalServerError)
            .sized_body(msg.len(), std::io::Cursor::new(msg))
            .ok()
    }
}
