use crate::{resources::quotes::model::Quote, server::responses::ApiError, App};
use axum::{
    extract::{Path, State},
    Json,
};

fn get(State(a): State<App>, Path(quote_id): Path<String>) -> Result<Json<Quote>, ApiError> {
    // Get quote from DB using ID
    // Shape should be a.<service>.get()

    // Return quote
    todo!()
}

fn list() {
    todo!()
}

fn update() {
    todo!()
}

fn delete() {
    todo!()
}
