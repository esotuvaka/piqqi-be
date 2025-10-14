use worker::D1Database;

use crate::{resources::line_items::model::LineItem, server::error::ApiError};

#[derive(Debug)]
pub struct LineItemRepo {
    db: D1Database,
}

impl LineItemRepo {
    pub fn new(db: D1Database) -> Self {
        LineItemRepo { db }
    }

    pub async fn create(&self, line_item: LineItem) -> Result<(), ApiError> {
        let query = r#"
            INSERT INTO line_items (

            )
        "#;
        Ok(())
    }
}
