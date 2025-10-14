use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub id: i64,
    pub customer_id: i64,
    pub entity_type: EntityType,
    pub entity_id: i64,
    pub name: String,
    pub sku: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub unit_cost: f64,
    pub profit: f64,
    pub margin: f64,
    pub discount: Option<f64>,
    pub discount_type: DiscountType,
    pub tax_rate: Option<f64>,
    pub notes: Option<String>,
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
