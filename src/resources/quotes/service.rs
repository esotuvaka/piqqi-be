use worker::Result;

use crate::resources::{
    line_items::{self, model::EntityType, repository::LineItemRepo},
    quotes::{
        self,
        model::{CreateRequest, Quote},
        repository::QuoteRepo,
    },
};

#[derive(Debug)]
pub struct QuoteService {
    quote_repo: QuoteRepo,
    line_item_repo: LineItemRepo,
}

impl QuoteService {
    pub fn new(quote_repo: QuoteRepo, line_item_repo: LineItemRepo) -> Self {
        QuoteService {
            quote_repo,
            line_item_repo,
        }
    }

    pub async fn create(
        &self,
        payload: quotes::model::CreateRequest,
        customer_id: String,
    ) -> Result<Quote> {
        // TODO: implement auth checks
        let quote = self
            .quote_repo
            .create(payload.clone(), customer_id.clone())
            .await?;
        let entity_type = EntityType::Quote;
        let _ = self
            .line_item_repo
            .create_many(payload.lines, entity_type, quote.clone().id, customer_id)
            .await?;
        Ok(quote)
    }

    pub async fn get(&self, quote_id: i64) -> Result<Quote> {
        self.quote_repo.get(quote_id).await
    }

    pub async fn list(&self, customer_id: i32) -> Result<Vec<Quote>> {
        self.quote_repo.list(customer_id).await
    }
}
