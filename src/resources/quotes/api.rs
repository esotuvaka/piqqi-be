use rand::{distr::Alphanumeric, Rng};
use validator::Validate;
use worker::{Error, Request, Response, RouteContext};

use crate::{
    cors,
    resources::quotes::{
        self,
        model::{CreateResponse, IncompletePayloadResponse, InvalidPayloadResponse, Quote},
    },
    App,
};

fn generate_id(prefix: &str) -> String {
    let rand_string: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}{}", prefix, rand_string)
}

pub async fn create(mut req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token
    let customer_id: String = "cus_39djwi10fhe2".to_string();

    let payload: quotes::model::CreateRequest = match req.json().await {
        Ok(p) => p,
        Err(e) => {
            let resp = IncompletePayloadResponse {
                message: "missing field".to_string(),
                error: e.to_string(),
            };
            return Ok(Response::from_json(&resp)?.with_status(400));
        }
    };
    match payload.validate() {
        Ok(_) => (),
        Err(e) => {
            let resp = InvalidPayloadResponse {
                message: "invalid payload".to_string(),
                errors: e,
            };
            return Ok(Response::from_json(&resp)?.with_status(422));
        }
    }

    let quote_id = generate_id("quo_");

    ctx.data
        .quote_service
        .create(payload, customer_id, quote_id.clone())
        .await
        .map_err(|e| Error::RustError(format!("creating quote: {e}").to_string()))?;

    Response::from_json(&CreateResponse { id: quote_id })?.with_cors(&cors())
}

pub async fn get(_req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token
    let customer_id: String = "cus_39djwi10fhe2".to_string();

    let quote_id = match ctx.param("quote_id") {
        Some(q) => q,
        None => return Response::error("missing quote_id", 400),
    };

    let quote: Quote = ctx
        .data
        .quote_service
        .get(quote_id.to_string())
        .await
        .map_err(|e| Error::RustError(format!("retrieving quote: {e}").to_string()))?;

    Response::from_json(&quote)
}

pub async fn list(_req: Request, ctx: RouteContext<App>) -> worker::Result<Response> {
    // TODO: grab customer ID from token
    // TODO: support pagination and query params for filtering

    let customer_id: String = "cus_39djwi10fhe2".to_string();

    let quotes: Vec<Quote> = ctx
        .data
        .quote_service
        .list(customer_id)
        .await
        .map_err(|e| Error::RustError(format!("listing quotes: {e}").to_string()))?;

    Response::from_json(&quotes)
}
