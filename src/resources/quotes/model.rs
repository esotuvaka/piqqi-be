use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::resources::line_items::{self, model::LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: Option<i64>,
    pub customer_id: i64,
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
    pub version: i64,
    pub lines: Vec<LineItem>,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateRequest {
    #[validate(length(min = 3))]
    pub contact_id: String,
    #[validate(length(min = 3))]
    pub sender_company: String,
    #[validate(length(min = 3))]
    pub sender_address: String,
    #[validate(length(min = 3))]
    pub sender_city_state_zip: String,
    #[validate(length(min = 3))]
    pub client_company: String,
    #[validate(length(min = 3))]
    pub client_address: String,
    #[validate(length(min = 3))]
    pub client_city_state_zip: String,
    #[validate(length(min = 3))]
    pub client_country: String,
    #[validate(length(min = 3))]
    pub quote_name: String,
    pub expires: i64, // timestamp or epoch
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    #[validate(length(min = 3))]
    pub payment_terms: String,
    #[validate(length(min = 3))]
    pub delivery_terms: String,
    #[validate(length(min = 3))]
    pub status: String,
    pub notes: String,
    pub message: String,
    pub tags: Vec<String>,
    pub version: i64,
    #[validate(length(min = 1))]
    pub lines: Vec<line_items::model::CreateRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub quote: Quote,
}
