use chrono::{DateTime, Datelike, Local, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateClientDTO {
    #[validate(length(min = 2, message = "Fist name is invalid"))]
    pub first_name: String,

    #[validate(length(min = 2, message = "Last name is invalid"))]
    pub last_name: String,

    #[validate(email(message = "Email is invalid"))]
    pub email: String,

    #[validate(custom(function = "validate_phone", message = "Invalid Phone Number"))]
    pub phone: String,

    #[validate(custom(function = "validate_cpf", message = "Invalid CPF"))]
    pub cpf: String,

    #[validate(length(min = 5, message = "The password must be at least 5 characters long"))]
    pub password: String,

    #[validate(custom(
        function = "validate_age_min",
        message = "Your age is less than 16 years old"
    ))]
    pub birth: DateTime<Utc>,
}

// functions validations
fn validate_age_min(birth: &DateTime<Utc>) -> Result<(), ValidationError> {
    let year = birth.year();
    let actual = Local::now().year();

    let age = actual - year;
    if age >= 16 {
        Ok(())
    } else {
        Err(ValidationError::new("Your age is less than 16 years old"))
    }
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let re = Regex::new("\\([0-9]{2}\\)[0-9]{5}-[0-9]{4}").unwrap();
    if re.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid Phone Number"))
    }
}

fn validate_cpf(cpf: &str) -> Result<(), ValidationError> {
    let re = Regex::new("[0-9]{3}\\.[0-9]{3}\\.[0-9]{3}-[0-9]{2}").unwrap();
    if re.is_match(cpf) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid CPF"))
    }
}
