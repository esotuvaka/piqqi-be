use std::sync::Arc;

use crate::{
    resources::quotes::model::{CreateResponse, Quote},
    server::error::ApiError,
    App,
};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};

#[debug_handler]
pub async fn create(
    State(a): State<Arc<App>>,
    Json(payload): Json<Quote>,
) -> Result<Json<CreateResponse>, ApiError> {
    // TODO: implement tag struct validation on mutation payloads

    let result = a.quote_service.create(payload).await;
    match result {
        Ok(quote) => Ok(Json(CreateResponse { quote })),
        Err(e) => Err(e),
    }
}

#[debug_handler]
pub async fn get(
    State(a): State<Arc<App>>,
    Path(quote_id): Path<i32>,
) -> Result<Json<Quote>, ApiError> {
    let quote = a.quote_service.get(quote_id).await;
    match quote {
        Ok(q) => Ok(Json(q)),
        Err(e) => Err(e),
    }
}

pub async fn list() {
    todo!()
}

pub async fn update() {
    todo!()
}

pub async fn delete() {
    todo!()
}
