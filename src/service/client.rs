use actix_web::web::Json;
use celes::Country;
use chrono::NaiveDate;
use core::str::FromStr;
use log::error;
use std::sync::Arc;
use tokio::sync::RwLock;

#[path = "../dto/mod.rs"]
mod dto;

#[path = "../model/mod.rs"]
mod model;

pub use dto::client::ClientBalanceSchema;
pub use dto::client::ClientSchema;
use dto::client::NewClientResponse;
pub use model::client::ClientModel;

#[derive(Clone)]
pub struct ClientService {
    clients: Arc<RwLock<Vec<ClientModel>>>,
}

impl ClientService {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn new_client(
        &self,
        client: Json<ClientSchema>,
    ) -> Result<NewClientResponse, String> {
        let client_count: i32;
        let document_number: serde_json::Number = match client.document_number.parse() {
            Ok(number) => number,
            Err(_) => {
                error!("Invalid document number at new_client: {:?}", client);
                return Err("Invalid document number".to_string());
            }
        };

        let country = match Country::from_str(&client.country) {
            Ok(country) => country,
            Err(_) => {
                error!("Invalid country at new_client: {:?}", client);
                return Err("Invalid country".to_string());
            }
        };

        {
            // get read lock
            let readable_clients: tokio::sync::RwLockReadGuard<'_, Vec<ClientModel>> =
                self.clients.read().await;

            let existing_client: Option<&ClientModel> =
                readable_clients.iter().find(|client: &&ClientModel| {
                    client.document_number == document_number && client.country == country
                });

            if existing_client.is_some() {
                error!("ClientExists at new_client: {:?}", existing_client);
                return Err("Client already exists".to_string());
            }
            client_count = (readable_clients.len() as i32) + 1;
        } // release read lock

        let birth_date: NaiveDate = match NaiveDate::parse_from_str(&client.birth_date, "%Y-%m-%d")
        {
            Ok(date) => date,
            Err(_) => {
                error!("Invalid date format at new_client: {:?}", client);
                return Err("Invalid date format".to_string());
            }
        };

        let new_client: ClientModel = ClientModel {
            id: client_count,
            client_name: client.client_name.clone(),
            birth_date,
            document_number,
            country,
        };

        {
            // get write lock
            let mut writable_clients: tokio::sync::RwLockWriteGuard<'_, Vec<ClientModel>> =
                self.clients.write().await;
            writable_clients.push(new_client.clone());
        } // release write lock

        let client_response: NewClientResponse = NewClientResponse {
            client_id: new_client.id.clone(),
        };

        Ok(client_response)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<ClientModel, String> {
        let readable_clients: tokio::sync::RwLockReadGuard<'_, Vec<ClientModel>> =
            self.clients.read().await;

        match readable_clients
            .iter()
            .find(|client: &&ClientModel| client.id == id)
        {
            Some(client) => Ok(client.clone()),
            None => {
                error!("Client not found at get_by_id: {:?}", id);
                return Err("Client not found".to_string());
            }
        }
    }
}
