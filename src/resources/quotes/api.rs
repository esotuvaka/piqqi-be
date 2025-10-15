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

pub async fn get() {}

pub async fn list() {
    todo!()
}

pub async fn update() {
    todo!()
}

pub async fn delete() {
    todo!()
}
