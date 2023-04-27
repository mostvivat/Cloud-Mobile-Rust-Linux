use actix_web::{App, HttpServer, middleware};
use env_logger::Env;

pub mod routes;
mod handlers;
mod models;
use crate::routes::{cart_routes, shipping_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

   HttpServer::new(|| {
       App::new()
            .wrap(middleware::Logger::default())
            .configure(cart_routes::config)
            .configure(shipping_routes::config)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
