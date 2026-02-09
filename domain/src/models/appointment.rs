use std::str::FromStr;

use super::{PaymentMethod, PaymentStatus, Service, User};
use crate::models::Log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppointmentId(Uuid);

impl AppointmentId {
    pub fn new(id: &str) -> Self {
        Self(Uuid::from_str(id).unwrap())
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Appointment<'a> {
    pub id: AppointmentId,

    pub client: User<'a>,
    pub professional: User<'a>,
    pub service: Service<'a>,

    pub sessions: Vec<Session>,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,

    pub nfe: &'a str,
    pub observations: &'a str,
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
