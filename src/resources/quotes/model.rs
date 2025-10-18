use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::resources::line_items::{self, model::LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub customer_id: String,
    pub contact_id: String,
    pub sender_company: String,
    pub sender_address: String,
    pub sender_city_state_zip: String,
    pub client_company: String,
    pub client_address: String,
    pub client_city_state_zip: String,
    pub client_country: String,
    pub quote_name: String,
    pub expires: String, // timestamp or epoch
    pub currency: String,
    pub payment_terms: String,
    pub delivery_terms: String,
    pub status: String,
    pub notes: String,
    pub message: String,
    pub tags: Vec<String>,
    pub version: i32,
    pub lines: Vec<LineItem>,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateRequest {
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub contact_id: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub sender_company: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub sender_address: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub sender_city_state_zip: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub client_company: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub client_address: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub client_city_state_zip: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub client_country: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub quote_name: String,
    pub expires: String, // Zulu/UTC time string
    #[validate(length(min = 3, max = 3, message = "must be a 3 letter country code"))]
    pub currency: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub payment_terms: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub delivery_terms: String,
    #[validate(length(min = 3, message = "must have at least 3 letters"))]
    pub status: String,
    pub notes: String,
    pub message: String,
    pub tags: Vec<String>,
    pub version: i32,
    #[validate(length(min = 1))]
    pub lines: Vec<line_items::model::CreateRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub quote: Quote,
}

#[derive(Debug, Serialize)]
pub struct InvalidPayloadResponse {
    pub message: String,
    pub errors: validator::ValidationErrors,
}

#[derive(Debug, Serialize)]
pub struct IncompletePayloadResponse {
    pub message: String,
    pub error: String,
}
