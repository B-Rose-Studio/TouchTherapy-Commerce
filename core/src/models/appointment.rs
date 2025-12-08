use super::{Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Appointment<'a> {
    pub id: Uuid,

    #[serde(borrow)]
    pub client: User<'a>,
    pub professional: User<'a>,
    pub service: Service<'a>,

    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
    pub status: AppointmentStatus,
    pub observations: &'a str,
}

#[derive(Serialize, Deserialize)]
pub enum AppointmentStatus {
    Pending,
    Confirmed,
    Completed,
    CanceledClient,
    CancelledAdmin,
}
