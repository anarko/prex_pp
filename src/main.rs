use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use env_logger::Env;
use log::debug;
use rust_dotenv::dotenv::DotEnv;

use service::payment::PaymentService;

use routes::{
    client::{client_balance, new_client},
    transaction::{new_credit_transaction, new_debit_transaction, store_balances},
};

mod routes;
mod service;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_IP: &'static str = "127.0.0.1";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let dotenv = DotEnv::new("");
    debug!("Loaded env: {:?}", dotenv.all_vars());

    let server_ip: String = dotenv
        .get_var("SERVER_IP".to_string())
        .unwrap_or(DEFAULT_IP.to_string());
    let server_port: u16 = dotenv
        .get_var("PORT".to_string())
        .unwrap()
        .parse::<u16>()
        .unwrap_or(DEFAULT_PORT);

    let payments: PaymentService = PaymentService::new();

    HttpServer::new(move || {
        App::new()
            .service((
                new_client,
                new_credit_transaction,
                new_debit_transaction,
                store_balances,
                client_balance,
            ))
            .app_data(web::Data::new(payments.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %i %b %D"))
    })
    .bind((server_ip, server_port))?
    .run()
    .await
}
