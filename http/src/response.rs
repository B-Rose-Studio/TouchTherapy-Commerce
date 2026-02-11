use std::fmt::Display;

use crate::meta::MetaInfos;
use domain::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(skip)]
    status_code: u16,
    success: bool,
    data: Option<Value>,
    meta: Option<MetaInfos>,
    error: Option<Error>,
}

impl Response {
    pub fn success(status_code: u16, data: &impl Serialize, version: &str) -> Self {
        Self {
            status_code,
            success: true,
            data: Some(serde_json::to_value(data).unwrap()),
            meta: Some(MetaInfos::new(version.to_owned())),
            error: None,
        }
    }

    pub fn error(
        status_code: u16,
        code: String,
        description: String,
        value: impl Serialize,
    ) -> Self {
        Self {
            status_code,
            success: false,
            data: None,
            meta: None,
            error: Some(Error::new(code, description, value)),
        }
    }
}

#[cfg(feature = "actix-web")]
use actix_web::{Responder, ResponseError, body::BoxBody, http::StatusCode};

#[cfg(feature = "actix-web")]
impl Responder for Response {
    type Body = BoxBody;

    fn customize(self) -> actix_web::CustomizeResponder<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponseBuilder::new(StatusCode::from_u16(self.status_code).unwrap())
            .json(self)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(feature = "actix-web")]
impl ResponseError for Response {
    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        actix_web::HttpResponseBuilder::new(StatusCode::from_u16(self.status_code).unwrap())
            .json(self)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).unwrap()
    }
}
