use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
}