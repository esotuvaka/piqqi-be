use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_service::Service;
use worker::*;

pub mod resources;
pub mod server;

use crate::resources::{
    line_items::repository::LineItemRepo,
    quotes::{self, repository::QuoteRepo, service::QuoteService},
};

// Initialize the server with all shared state, SDKs, etc.
// that can be passed into API handlers
#[derive(Clone, Debug)]
struct App {
    quote_service: Arc<QuoteService>,
}

fn router(app: App) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/quotes", post(quotes::api::create))
        .route("/quotes", get(quotes::api::list))
        .route("/quotes/:id", get(quotes::api::get))
        .route("/quotes/:id", put(quotes::api::update))
        .route("/quotes/:id", delete(quotes::api::delete))
        .with_state(app)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let quote_repo = QuoteRepo::new(env.d1("piqqi").unwrap());
    let line_items_repo = LineItemRepo::new(env.d1("piqqi").unwrap());
    let quote_service = QuoteService::new(quote_repo, line_items_repo);

    let app = App {
        quote_service: quote_service.into(),
    };

    let repos = Repos::new();
    let svcs = Services::new();
    let h = Handlers::new();

    Ok(router(app).call(req).await?)
}

struct Repos {}
impl Repos {
    fn new() -> Self {
        Repos {}
    }
}

struct Services {}
impl Services {
    fn new() -> Self {
        Services {}
    }
}

struct Handlers {}
impl Handlers {
    fn new() -> Self {
        Handlers {}
    }
}
