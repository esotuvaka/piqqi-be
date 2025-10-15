use worker::Result;

use crate::resources::{
    line_items::repository::LineItemRepo,
    quotes::{model::Quote, repository::QuoteRepo},
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

    pub async fn create(&self, quote: Quote) -> Result<Quote> {
        // TODO: implement auth checks
        let lines = &quote.clone().lines;
        self.line_item_repo.create_many(&lines).await;
        self.quote_repo.create(quote).await
    }

    pub async fn get(&self, quote_id: i32) -> Result<Quote> {
        self.quote_repo.get(quote_id).await
    }

    pub async fn list(&self, customer_id: i32) -> Result<Vec<Quote>> {
        self.quote_repo.list(customer_id).await
    }
}
