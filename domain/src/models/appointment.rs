use super::{PaymentMethod, PaymentStatus, Service, User};
use crate::models::Log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Appointment {
    pub id: AppointmentId,

    pub client: User,
    pub professional: User,
    pub service: Service,

    pub sessions: Vec<Session>,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,

    pub nfe: String,
    pub observations: String,
    pub log: Log,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Session {
    pub date_start: Option<DateTime<Utc>>,
    pub date_end: Option<DateTime<Utc>>,
    pub status: AppointmentStatus,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum AppointmentStatus {
    Pending,
    Scheduled,
    Canceled,
    Completed,
    NoShow,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppointmentId(Uuid);

impl AppointmentId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

impl Deref for AppointmentId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
