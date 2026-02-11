use domain::error::ErrorTrait;
use serde::Serialize;

mod builder;
mod service;

pub use builder::*;
pub use service::*;

#[derive(Debug, Serialize)]
pub struct SendEmailError(pub String);

impl ErrorTrait for SendEmailError {
    fn error<T: Sized + Serialize>(&self) -> domain::error::Error {
        domain::error::Error::new("SEND_EMAIL_FAIL", "could not send email", self.0.clone())
    }
}

pub struct From {
    pub name: String,
    pub email: String,
}

pub struct To {
    pub name: String,
    pub email: String,
}

pub struct Cred {
    pub app_key: String,
}
