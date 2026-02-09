use super::{PaymentMethod, PaymentStatus, Service, User};
use crate::models::Log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Appointment<'a> {
    pub id: Uuid,

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

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub date_start: Option<DateTime<Utc>>,
    pub date_end: Option<DateTime<Utc>>,
    pub status: AppointmentStatus,
}

#[derive(Serialize, Deserialize)]
pub enum AppointmentStatus {
    Pending,
    Scheduled,
    Canceled,
    Completed,
    NoShow,
}
