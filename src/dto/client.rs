use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientSchema {
    pub client_id: Option<i32>,
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ClientBalanceSchema {
    pub client: ClientSchema,
    pub balance: Decimal,
}

#[derive(Serialize, Clone, Debug)]
pub struct NewClientResponse {
    pub client_id: i32,
}
