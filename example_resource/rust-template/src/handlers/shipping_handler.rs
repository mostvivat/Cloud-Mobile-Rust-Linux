use actix_web::{get, Responder, HttpResponse};

#[get("/shipping")]
async fn get_shipping() -> impl Responder {
    HttpResponse::Ok().json("Hello Shipping!")
}