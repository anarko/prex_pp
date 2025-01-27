use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct BalanceModel {
    pub client_id: i32,
    pub balance: Decimal,
    pub last_updated: NaiveDateTime,
}
