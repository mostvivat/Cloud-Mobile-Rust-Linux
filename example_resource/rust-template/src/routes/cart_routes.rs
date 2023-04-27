use actix_web::web;
use crate::handlers::cart_handler::{get_cart, get_cart_item_by_id} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_cart)
        .service(get_cart_item_by_id);
}