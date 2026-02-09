use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentMethod {
    Pix,
    CreditcCard,
    DebitCard,
    Boleto,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Canceled,
    Reversed,
}
