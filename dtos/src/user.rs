use chrono::{DateTime, Datelike, Local, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateClientDTO {
    #[validate(length(min = 2))]
    pub first_name: String,

    #[validate(length(min = 2))]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "validate_phone"))]
    pub phone: String,

    #[validate(custom(function = "validate_cpf"))]
    pub cpf: String,

    #[validate(length(min = 5))]
    pub password: String,

    #[validate(custom(function = "validate_age_min"))]
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
        return Err(ValidationError::new("Your age is less than 16 years old."));
    }
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let re = Regex::new("([0-9]{2})[0-9]{5}-[0-9]{4}").unwrap();
    if re.is_match(phone) {
        Ok(())
    } else {
        return Err(ValidationError::new("Invalid Phone Number."));
    }
}

fn validate_cpf(cpf: &str) -> Result<(), ValidationError> {
    let re = Regex::new("[0-9]{3}.[0-9]{3}.[0-9]{3}-[0-9]{2}").unwrap();
    if re.is_match(cpf) {
        Ok(())
    } else {
        return Err(ValidationError::new("Invalid CPF."));
    }
}
