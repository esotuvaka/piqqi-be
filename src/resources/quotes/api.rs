use worker::{Error, Request, Response, RouteContext};

use crate::{resources::quotes::model::Quote, App};

pub async fn create(mut req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token

    let payload: Quote = req.json().await.unwrap();

    let quote = ctx
        .data
        .quote_repo
        .create(payload)
        .await
        .map_err(|_e| Error::RustError("creating quote".to_string()))?;

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
        .quote_repo
        .get(quote_id)
        .await
        .map_err(|_e| Error::RustError("retrieving quote".to_string()))?;

    Response::from_json(&quote)
}

pub async fn list(_req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token
    // TODO: support pagination and query params for filtering

    let customer_id = 1;

    let quotes: Vec<Quote> = ctx
        .data
        .quote_repo
        .list(customer_id)
        .await
        .map_err(|e| Error::RustError(format!("listing quotes: {}", e).to_string()))?;

    Response::from_json(&quotes)
}
