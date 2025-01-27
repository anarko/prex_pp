use actix_web::{get, post, web, HttpResponse, Responder};

use crate::service::{client::ClientSchema, payment::PaymentService};

#[post("/new_client")]
pub async fn new_client(
    req_body: web::Json<ClientSchema>,
    payments: web::Data<PaymentService>,
) -> impl Responder {
    match payments.client_service.new_client(req_body).await {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/client_balance/{user_id}")]
pub async fn client_balance(
    path: web::Path<i32>,
    app_data: web::Data<PaymentService>,
) -> impl Responder {
    let user_id: i32 = path.into_inner();

    match app_data.get_client_balance(user_id.clone()).await {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
