use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;

use crate::service::payment::{PaymentService, TransactionSchema};

#[post("/new_credit_transaction")]
pub async fn new_credit_transaction(
    transaction: web::Json<TransactionSchema>,
    payments: web::Data<PaymentService>,
) -> impl Responder {
    match payments.process(transaction).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[post("/new_debit_transaction")]
pub async fn new_debit_transaction(
    transaction: web::Json<TransactionSchema>,
    payments: web::Data<PaymentService>,
) -> impl Responder {
    match payments.process(transaction).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/store_balances")]
pub async fn store_balances(payments: web::Data<PaymentService>) -> impl Responder {
    match payments
        .conciliation_service
        .do_conciliation(payments.balances.read().await.clone())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            error!("store_balances failed");
            return HttpResponse::InternalServerError().body(err);
        }
    }
    payments.reset_balances().await;
    return HttpResponse::Ok().finish();
}
