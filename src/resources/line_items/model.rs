use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub id: String,
    pub customer_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub name: String,
    pub sku: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub unit_cost: f64,
    pub discount: Option<f64>,
    pub discount_type: DiscountType,
    pub notes: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiscountType {
    Percent,
    Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Quote,
    SalesOrder,
    Fulfillment,
    Shipping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub sku: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub unit_cost: f64,
    pub profit: f64,
    pub margin: f64,
    pub discount: f64,
    pub discount_type: DiscountType,
    pub notes: String,
}
