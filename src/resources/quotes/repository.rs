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
        let query = "INSERT INTO quotes () VALUES (?1, ?2, ?3)";
        let statement = self.db.prepare(query).bind(&[])?;
        let result = statement.run().await?;
        console_log!("result: {:?}", result.success());
        match result.success() {
            true => Ok(()),
            false => Err(ApiError::InternalServerError),
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
