use chrono::prelude::*;
use log::debug;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::spawn;
use tokio::sync::RwLock;

use super::payment::BalanceModel;

#[derive(Clone, Debug)]
pub struct ConciliationService {
    las_conciliation_number: Arc<RwLock<i32>>,
}

impl ConciliationService {
    pub fn new() -> Self {
        Self {
            las_conciliation_number: Arc::new(RwLock::new(1)),
        }
    }
    pub async fn increase_conciliation_number(&self) {
        let mut las_conciliation_number: tokio::sync::RwLockWriteGuard<'_, i32> =
            self.las_conciliation_number.write().await;
        *las_conciliation_number += 1;
    }

    pub async fn do_conciliation(&self, client_balances: Vec<BalanceModel>) -> Result<(), String> {
        let conciliation_number = {
            let las_conciliation_number: tokio::sync::RwLockReadGuard<'_, i32> =
                self.las_conciliation_number.read().await;
            *las_conciliation_number
        };

        debug!(
            "ConciliationService::do_conciliation {:?}",
            conciliation_number
        );

        if client_balances.is_empty() {
            debug!("ConciliationService::do_conciliation NO CLIENTS");
            return Ok(());
        }

        // spawn thread to write conciliation file
        spawn(async move {
            debug!("Starting conciliation file write");
            let now = Local::now().format("%Y%m%d").to_string();
            let file_name = format!("{}_{}.dat", now, conciliation_number.to_string());
            let mut file = match File::create(file_name).await {
                Ok(file) => file,
                Err(e) => {
                    debug!("Failed to create file: {:?}", e);
                    return;
                }
            };

            for client in client_balances {
                let balance: String = format!("{:02} {}\n", client.client_id, client.balance);
                match file.write_all(balance.as_bytes()).await {
                    Ok(_) => (),
                    Err(e) => {
                        debug!("Failed to write to file: {:?}", e);
                        return;
                    }
                }
            }

            debug!("Conciliation file write  DONE");
        });

        debug!("ConciliationService::do_conciliation DONE");
        self.increase_conciliation_number().await;
        Ok(())
    }
}
