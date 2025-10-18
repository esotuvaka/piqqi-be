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
        quote_id: String,
    ) -> Result<()> {
        // TODO: implement auth checks
        let entity_type = EntityType::Quote;
        self.quote_repo
            .create(payload.clone(), customer_id.clone(), quote_id.clone())
            .await?;
        self.line_item_repo
            .create_many(payload.lines, entity_type, quote_id.clone(), customer_id)
            .await?;
        Ok(())
    }

    pub async fn get(&self, quote_id: String) -> Result<Quote> {
        self.quote_repo.get(quote_id).await
    }

    pub async fn list(&self, customer_id: String) -> Result<Vec<Quote>> {
        self.quote_repo.list(customer_id).await
    }
}
