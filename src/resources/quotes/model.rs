use serde::{Deserialize, Serialize};

use crate::resources::line_items::model::LineItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: i32,
    pub customer_id: i32,
    pub contact_id: String,
    pub sender_company: String,
    pub sender_address: String,
    pub sender_city_state_zip: String,
    pub client_company: String,
    pub client_address: String,
    pub client_city_state_zip: String,
    pub client_country: String,
    pub quote_name: String,
    pub expires: i64, // timestamp or epoch
    pub currency: String,
    pub payment_terms: String,
    pub delivery_terms: String,
    pub status: String,
    pub notes: String,
    pub message: String,
    pub tags: Vec<String>,
    pub version: i32,
    pub lines: Vec<LineItem>,
    pub updated_at: i32,
    pub created_at: i32,
}
