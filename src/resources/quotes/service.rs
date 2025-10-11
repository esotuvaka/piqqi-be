use anyhow::Result;

use crate::resources::quotes::{model::Quote, repository::QuoteRepo};

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
    pub async fn get(&self, quote_id: String) -> Result<Quote> {
        self.quote_repo.get(quote_id)
    }

    pub async fn list() {}
}
