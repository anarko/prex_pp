use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionSchema {
    pub client_id: i32,
    pub credit_amount: Option<Decimal>,
    pub debit_amount: Option<Decimal>,
}
