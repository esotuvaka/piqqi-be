use worker::{console_log, D1Database, Result};

use crate::resources::line_items::{
    self,
    model::{DiscountType, EntityType, LineItem},
};

#[derive(Debug)]
pub struct LineItemRepo {
    db: D1Database,
}

impl LineItemRepo {
    pub fn new(db: D1Database) -> Self {
        LineItemRepo { db }
    }

    pub async fn create_many(
        &self,
        line_items: Vec<line_items::model::CreateRequest>,
        entity_type: EntityType,
        entity_id: String,
        customer_id: String,
    ) -> Result<()> {
        let mut statements = vec![];
        for li in line_items {
            let entity_type = match entity_type {
                EntityType::Quote => "quote",
                EntityType::SalesOrder => "salesorder",
                EntityType::Fulfillment => "fulfillment",
                EntityType::Shipping => "shipping",
            };
            let discount_type = match li.discount_type {
                DiscountType::Percent => "percent",
                DiscountType::Value => "value",
            };

            let statement = self
                .db
                .prepare(
                    r#"INSERT INTO line_items (
                    customer_id, entity_type, entity_id, name, sku, quantity,
                    unit_price, unit_cost, discount, discount_type, notes
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"#,
                )
                .bind(&[
                    customer_id.clone().into(),
                    entity_type.into(),
                    entity_id.clone().into(),
                    li.name.clone().into(),
                    li.sku.clone().into(),
                    li.quantity.into(),
                    li.unit_price.into(),
                    li.unit_cost.into(),
                    li.discount.into(),
                    discount_type.into(),
                    li.notes.clone().into(),
                ])?;
            statements.push(statement)
        }

        let results = self.db.batch(statements).await;
        match results {
            Ok(_) => Ok(()),
            Err(e) => Err(worker::Error::RustError(e.to_string())),
        }
    }

    pub async fn list(&self, customer_id: i32, entity_id: i32) -> Result<Vec<LineItem>> {
        let query =
            "SELECT * FROM line_items WHERE customer_id = ?1 AND entity_id = ?2".to_string();
        let statement = self
            .db
            .prepare(query)
            .bind(&[customer_id.to_string().into(), entity_id.to_string().into()])
            .expect("failed to bind query params");
        let line_items = statement.all().await?.results::<LineItem>()?;
        Ok(line_items)
    }
}
