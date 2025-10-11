use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub name: String,
    pub sku: String,
    pub quantity: i32,
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
pub struct Quote {
    pub sender_company: String,
    pub sender_address: String,
    pub sender_city_state_zip: String,
    pub client_company: String,
    pub quote_name: String,
    pub contact_id: String,
    pub expires: i64, // timestamp or epoch
    pub currency: String,
    pub address_line_1: String,
    pub city: String,
    pub country: String,
    pub payment_terms: String,
    pub delivery_terms: String,
    pub status: String,
    pub notes: String,
    pub message: String,
    pub tags: Vec<String>,
    pub version: i32,
    pub lines: Vec<LineItem>,
}
