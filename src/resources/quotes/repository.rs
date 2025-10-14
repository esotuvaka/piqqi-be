use serde_json::to_string;
use worker::{console_log, D1Database};

use crate::{resources::quotes::model::Quote, server::error::ApiError};

/// Repositories _only_ perform data access. They don't care for auth or business logic,
/// which should be handled by services
#[derive(Debug)]
pub struct QuoteRepo {
    db: D1Database,
}

impl QuoteRepo {
    pub fn new(db: D1Database) -> Self {
        QuoteRepo { db }
    }

    pub async fn create(&self, quote: Quote) -> Result<(), ApiError> {
        // Serialize tags to JSON
        let tags_json =
            to_string(&quote.tags).map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Prepare the SQL query with all columns except id (autoincrement)
        let query = r#"
            INSERT INTO quotes (
                customer_id,
                contact_id,
                sender_company,
                sender_address,
                sender_city_state_zip,
                client_company,
                client_address,
                client_city_state_zip,
                client_country,
                quote_name,
                expires,
                currency,
                payment_terms,
                delivery_terms,
                status,
                notes,
                message,
                tags,
                version,
                created_at,
                updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15,
                ?16, ?17, ?18, ?19,
                strftime('%s','now'), strftime('%s','now')
            )
        "#;

        let statement = self.db.prepare(query).bind(&[
            quote.customer_id.into(),
            quote.contact_id.into(),
            quote.sender_company.into(),
            quote.sender_address.into(),
            quote.sender_city_state_zip.into(),
            quote.client_company.into(),
            quote.client_address.into(),
            quote.client_city_state_zip.into(),
            quote.client_country.into(),
            quote.quote_name.into(),
            quote.expires.into(),
            quote.currency.into(),
            quote.payment_terms.into(),
            quote.delivery_terms.into(),
            quote.status.into(),
            quote.notes.into(),
            quote.message.into(),
            tags_json.into(),
            quote.version.into(),
        ])?;

        let result = statement.run().await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError::InternalServerError(e.to_string())),
        }
    }

    pub async fn get(&self, quote_id: String) -> Result<Quote, ApiError> {
        let query = "SELECT q.*, li.* FROM quotes q LEFT JOIN line_items li ON li.quote_id = q.id WHERE q.id = $1;".to_string();
        let statement = self
            .db
            .prepare(query)
            .bind(&[quote_id.to_string().into()])
            .expect("failed to bind query params");
        let maybe_quote = statement.first::<Quote>(None).await?;
        match maybe_quote {
            Some(q) => Ok(q),
            None => Err(ApiError::NotFound),
        }
    }

    pub async fn list(&self, customer_id: String) -> Result<Vec<Quote>, ApiError> {
        let query = "SELECT q.*, li.* FROM quotes q LEFT JOIN line_items li ON li.quote_id = q.id WHERE customer_id = $1".to_string();
        let statement = self
            .db
            .prepare(query)
            .bind(&[customer_id.to_string().into()])
            .expect("failed to bind query params");
        let quotes = statement.all().await?.results::<Quote>()?;
        Ok(quotes)
    }
}
