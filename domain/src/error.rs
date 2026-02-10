use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Error<T: Sized + Serialize> {
    pub code: String,
    pub description: String,
    pub content: T,
}

impl<T: Sized + Serialize> Error<T> {
    pub fn new(code: impl Into<String>, description: impl Into<String>, content: T) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            content,
        }
    }
}

pub trait ErrorTrait {
    type Out: Sized + Serialize;

    fn error<T: Sized + Serialize>(&self) -> Error<Self::Out>;
}
