use actix_web::{web, get, Responder, HttpResponse};
use log::{info, debug};
use serde_json::json;

use crate::models::product::Product;


#[get("/cart")]
async fn get_cart() -> impl Responder {
    HttpResponse::Ok().json("Hello Jumpbox!")
}

#[get("/cart/{cart_item}")]
async fn get_cart_item_by_id(cart_item: web::Path<i32>) -> impl Responder {
    info!("get cart by id");
    debug!("id: {}",cart_item);
    let id: i32 = cart_item.to_string().parse().unwrap();
    let product_item = vec![
        Product {
            id: id,
            name: "iphone".to_string()
        },
        Product {
            id: 123,
            name: "ipad".to_string()
        }
    ];

    let response_body = json!(product_item);
  
    HttpResponse::Ok().json(response_body)
}