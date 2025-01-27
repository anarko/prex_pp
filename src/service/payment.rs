use actix_web::web::Json;
use log::{debug, error};
use rust_decimal::Decimal;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use super::{
    client::{ClientBalanceSchema, ClientModel, ClientSchema, ClientService},
    conciliation::ConciliationService,
};
pub use dto::transaction::TransactionSchema;
pub use model::balance::BalanceModel;

#[path = "../dto/mod.rs"]
mod dto;

#[path = "../model/mod.rs"]
mod model;

#[derive(Clone)]
pub struct PaymentService {
    pub balances: Arc<RwLock<Vec<BalanceModel>>>,
    pub client_service: ClientService,
    pub conciliation_service: ConciliationService,
}

impl PaymentService {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(Vec::new())),
            client_service: ClientService::new(),
            conciliation_service: ConciliationService::new(),
        }
    }

    pub async fn process(
        &self,
        transaction: Json<TransactionSchema>,
    ) -> Result<BalanceModel, String> {
        let client_id: i32 = transaction.client_id.clone();
        let amount: Decimal = if transaction.credit_amount.is_some() {
            transaction.credit_amount.unwrap()
        } else if transaction.debit_amount.is_some() {
            -transaction.debit_amount.unwrap()
        } else {
            // transaction without amount is not allowed
            return Err("Invalid transaction".to_string());
        };

        if amount == 0.into() {
            error!("Invalid amount: {:?}", amount);
            return Err("Invalid amount".to_string());
        }

        debug!("Processing transaction: {:?}", transaction);
        return self.update_or_create(client_id, amount).await;
    }

    async fn update_or_create(
        &self,
        client_id: i32,
        amount: Decimal,
    ) -> Result<BalanceModel, String> {
        self.client_service.get_by_id(client_id.clone()).await?;

        // get write lock
        let mut writable_balances: tokio::sync::RwLockWriteGuard<'_, Vec<BalanceModel>> =
            self.balances.write().await;
        let balance: Option<&mut BalanceModel> = writable_balances
            .iter_mut()
            .find(|b: &&mut BalanceModel| b.client_id == client_id);

        if balance.is_none() {
            let new_balance: BalanceModel = BalanceModel {
                client_id: client_id.clone(),
                balance: amount,
                last_updated: chrono::Utc::now().naive_utc(),
            };

            writable_balances.push(new_balance.clone());
            debug!("New balance: {:?}", new_balance);
            return Ok(new_balance);
        }

        let balance: &mut BalanceModel = balance.unwrap();
        let new_balance: Decimal = balance.balance + amount;
        balance.balance = new_balance;
        balance.last_updated = chrono::Utc::now().naive_utc();
        debug!("Updated balance: {:?}", balance);
        return Ok(balance.clone());
    }

    async fn find_by_client_id(&self, client_id: i32) -> Option<BalanceModel> {
        let readable_balances: tokio::sync::RwLockReadGuard<'_, Vec<BalanceModel>> =
            self.balances.read().await;
        let balance: Option<&BalanceModel> = readable_balances
            .iter()
            .find(|b: &&BalanceModel| b.client_id == client_id);
        match balance {
            Some(b) => Some(b.clone()),
            None => None,
        }
    }

    pub async fn get_client_balance(&self, client_id: i32) -> Result<ClientBalanceSchema, String> {
        let client: ClientModel = self.client_service.get_by_id(client_id.clone()).await?;
        let balance: Decimal = match self.find_by_client_id(client_id.clone()).await {
            Some(b) => b.balance,
            None => Decimal::from_str("0").unwrap(),
        };

        let client_balance: ClientBalanceSchema = ClientBalanceSchema {
            client: ClientSchema {
                client_id: Some(client.id.clone()),
                client_name: client.client_name.clone(),
                birth_date: client.birth_date.to_string(),
                document_number: client.document_number.to_string(),
                country: client.country.to_string(),
            },
            balance,
        };
        Ok(client_balance)
    }

    pub async fn reset_balances(&self) {
        // get write lock
        let mut writable_balances: tokio::sync::RwLockWriteGuard<'_, Vec<BalanceModel>> =
            self.balances.write().await;
        for balance in writable_balances.iter_mut() {
            balance.balance = Decimal::from_str("0").unwrap();
        }
    }
}
