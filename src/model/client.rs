use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientModel {
    pub id: i32,
    pub client_name: String,
    pub birth_date: NaiveDate,
    pub document_number: Number,
    pub country: celes::Country,
}
