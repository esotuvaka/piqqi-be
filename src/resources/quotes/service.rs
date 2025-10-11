use crate::{
    resources::quotes::{model::Quote, repository::QuoteRepo},
    server::error::ApiError,
};

#[derive(Debug)]
pub struct QuoteService {
    quote_repo: QuoteRepo,
}

impl QuoteService {
    pub fn new(quote_repo: QuoteRepo) -> Self {
        QuoteService { quote_repo }
    }

    // NOTE: use anyhow errors here. If we bail, its an internal
    // error, as validation and auth errors will be caught
    // by the endpoint handler
    pub async fn get(&self, quote_id: String) -> Result<Quote, ApiError> {
        self.quote_repo.get(quote_id).await
    }

    pub async fn list() {}
}
