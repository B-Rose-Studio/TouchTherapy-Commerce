use figment::{
    Figment,
    providers::{Env, Format, Json},
};
use serde::{Deserialize, Serialize};

mod email;
mod local_storage;
mod s3_storage;
mod security;
mod surrealdb;

pub use email::*;
pub use local_storage::*;
pub use s3_storage::*;
pub use security::*;
pub use surrealdb::*;

pub fn load_config<'a, T: Serialize + Deserialize<'a>>(prefix: &'a str, split: &'a str) -> T {
    Figment::new()
        .merge(Env::prefixed(prefix).split(split))
        .merge(Json::file("config.json"))
        .extract::<T>()
        .expect("Invalid Configuration")
}
