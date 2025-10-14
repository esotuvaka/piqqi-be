use crate::{
    resources::{
        line_items::repository::LineItemRepo,
        quotes::{model::Quote, repository::QuoteRepo},
    },
    server::error::ApiError,
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

    pub async fn create(&self, quote: Quote) -> Result<(), ApiError> {
        self.quote_repo.create(quote).await;
        self.line_item_repo.create(quote).await
    }

    pub async fn get(&self, quote_id: String) -> Result<Quote, ApiError> {
        self.quote_repo.get(quote_id).await
    }

    pub async fn list(&self, customer_id: String) -> Result<Vec<Quote>, ApiError> {
        self.quote_repo.list(customer_id).await
    }
}
