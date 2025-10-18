use serde_json::to_string;
use worker::*;

use crate::resources::quotes::model::{CreateRequest, Quote};

/// Repositories _only_ perform data access. They don't care for auth or business logic,
/// which should be handled by services
#[derive(Debug)]
pub struct QuoteRepo {
    db: D1Database,
}

fn print_type_of<T>(_: &T) {
    console_log!("{}", std::any::type_name::<T>());
}

impl QuoteRepo {
    pub fn new(db: D1Database) -> Self {
        QuoteRepo { db }
    }

    pub async fn create(
        &self,
        quote: CreateRequest,
        customer_id: String,
        quote_id: String,
    ) -> Result<()> {
        console_log!("quote: {:?}", quote);
        console_log!("customer_id: {customer_id}");

        let query = r#"
            INSERT INTO quotes (
                id,
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
                version
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15,
                ?16, ?17, ?18, ?19, ?20
            );
        "#;

        let tags_json = to_string(&quote.tags)
            .map_err(|_e| Error::RustError("converting tags array to string".to_string()))?;
        print_type_of(&tags_json);
        console_log!("tags_json: {tags_json}");

        let statement = self
            .db
            .prepare(query)
            .bind(&[
                quote_id.into(),
                customer_id.into(),
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
            ])
            .expect("bind query params");

        let result = statement.run().await?;
        match result.success() {
            true => Ok(()),
            false => Err(worker::Error::RustError(
                result.error().expect("error to have occurred"),
            )),
        }
    }

    pub async fn get(&self, quote_id: String) -> Result<Quote> {
        let query = "SELECT q.*, li.* FROM quotes q LEFT JOIN line_items li ON li.quote_id = q.id WHERE q.id = $1;".to_string();
        let statement = self
            .db
            .prepare(query)
            .bind(&[quote_id.into()])
            .expect("failed to bind query params");
        let maybe_quote = statement.first::<Quote>(None).await?;
        match maybe_quote {
            Some(q) => Ok(q),
            None => Err(Error::RustError("quote missing".to_string())),
        }
    }

    pub async fn list(&self, customer_id: String) -> Result<Vec<Quote>> {
        let query = "SELECT q.*, li.* FROM quotes q LEFT JOIN line_items li ON li.entity_id = q.id WHERE li.customer_id = ?1 AND li.customer_id = q.customer_id".to_string();
        let statement = self
            .db
            .prepare(query)
            .bind(&[customer_id.into()])
            .expect("failed to bind query params");
        let quotes = statement.all().await?.results::<Quote>()?;
        Ok(quotes)
    }
}
