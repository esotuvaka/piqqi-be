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
                customer_id,
                entity_type,
                entity_id,
                name,
                sku,
                quantity,
                unit_price,
                unit_cost,
                profit,
                margin,
                discount,
                discount_type,
                tax_rate,
                notes
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        "#;

        let statement = self.db.prepare(query).bind(&[
            line_item.customer_id.into(),
            serde_json::to_string(&line_item.entity_type)
                .map_err(|_| ApiError::InternalServerError)?
                .into(),
            line_item.entity_id.into(),
            line_item.name.into(),
            line_item.sku.into(),
            line_item.quantity.into(),
            line_item.unit_price.into(),
            line_item.unit_cost.into(),
            line_item.profit.into(),
            line_item.margin.into(),
            line_item.discount.map(Into::into).unwrap_or_default(),
            serde_json::to_string(&line_item.discount_type)
                .map_err(|_| ApiError::InternalServerError)?
                .into(),
            line_item.tax_rate.map(Into::into).unwrap_or_default(),
            line_item.notes.unwrap_or_default().into(),
        ])?;

        let result = statement.run().await.map_err(ApiError::from)?;

        if result.success() {
            Ok(())
        } else {
            Err(ApiError::InternalServerError)
        }
    }

    pub async fn create_many(&self, line_items: &[LineItem]) -> Result<(), ApiError> {
        let statements = vec![];
        for li in line_items {
            let statement = self
                .db
                .prepare(
                    r#"INSERT INTO line_items (
                    customer_id, entity_type, entity_id, name, sku, quantity,
                    unit_price, unit_cost, profit, margin, discount, discount_type, tax_rate, notes
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"#,
                )
                .bind(&[
                    li.customer_id.into(),
                    serde_json::to_string(&li.entity_type)
                        .unwrap_or_default()
                        .into(),
                    li.entity_id.into(),
                    li.name.into(),
                    li.sku.into(),
                    li.quantity.into(),
                    li.unit_price.into(),
                    li.unit_cost.into(),
                    li.profit.into(),
                    li.margin.into(),
                    li.discount.into(),
                    serde_json::to_string(&li.discount_type)
                        .unwrap_or_default()
                        .into(),
                    li.tax_rate.into(),
                    li.notes.into(),
                ])?;
            statements.push(statement)
        }

        let results = self.db.batch(statements).await;
    }
}
