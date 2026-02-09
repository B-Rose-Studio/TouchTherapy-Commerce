mod user;

pub use user::*;
use validator::Validate;

// Functions
pub fn is_valid(dto: &impl Validate) -> Result<(), Vec<String>> {
    match dto.validate() {
        Ok(_) => Ok(()),
        Err(err) => {
            let mut msg = vec![];

            for (field, errors) in err.field_errors() {
                if let Some(error) = errors.first() {
                    let error_msg = format!(
                        "{}: {}",
                        field,
                        error.message.clone().unwrap_or(error.code.clone())
                    );

                    msg.push(error_msg);
                }
            }

            Err(msg)
        }
    }
}

// Traits
pub trait ToEntity {
    type Entity: Validate;

    fn to(&self) -> Self::Entity;
}

pub trait MergeWithEntity {
    type Entity: Validate;

    fn merge(&self, entity: &Self::Entity) -> Self::Entity;
}

pub trait FromEntity {
    type Entity: Validate;

    fn from(entity: &Self::Entity) -> Self;
}
