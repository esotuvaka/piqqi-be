use validator::Validate;
use worker::{Error, Request, Response, RouteContext};

use crate::{
    resources::quotes::{self, model::Quote},
    App,
};

pub async fn create(mut req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token
    let customer_id: i64 = 1;

    let payload: quotes::model::CreateRequest = match req.json().await {
        Ok(p) => p,
        Err(_) => return Response::error("invalid payload; ensure all fields are populated", 400),
    };
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Response::error(format!("invalid payload {e}"), 400),
    }

    let quote = ctx
        .data
        .quote_service
        .create(payload, customer_id)
        .await
        .map_err(|e| Error::RustError(format!("creating quote: {e}").to_string()))?;

    Response::from_json(&quote)
}

pub async fn get(_req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token

    let maybe_quote_id = match ctx.param("quote_id") {
        Some(q) => q,
        None => return Response::error("missing quote_id", 400),
    };
    let quote_id = match maybe_quote_id.parse::<i64>() {
        Ok(q) => q,
        Err(_) => return Response::error("invalid quote_id", 400),
    };

    let quote: Quote = ctx
        .data
        .quote_service
        .get(quote_id)
        .await
        .map_err(|e| Error::RustError(format!("retrieving quote: {e}").to_string()))?;

    Response::from_json(&quote)
}

pub async fn list(_req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token
    // TODO: support pagination and query params for filtering

    let customer_id = 1;

    let quotes: Vec<Quote> = ctx
        .data
        .quote_service
        .list(customer_id)
        .await
        .map_err(|e| Error::RustError(format!("listing quotes: {e}").to_string()))?;

    Response::from_json(&quotes)
}
