use worker::*;

pub mod resources;
pub mod server;

use crate::resources::{
    line_items::repository::LineItemRepo,
    quotes::{self, repository::QuoteRepo, service::QuoteService},
};

pub struct App {
    quote_service: QuoteService,
}

const ALLOWED_ORIGINS: [&str; 1] = ["http://localhost:3000"];
const ALLOWED_METHODS: [Method; 6] = [
    Method::Get,
    Method::Post,
    Method::Put,
    Method::Patch,
    Method::Delete,
    Method::Delete,
];

// TODO: make this constant!
fn cors() -> Cors {
    Cors::new()
        .with_max_age(86400)
        .with_origins(ALLOWED_ORIGINS)
        .with_methods(ALLOWED_METHODS)
        .with_allowed_headers(vec!["Content-Type", "Authorization"])
        .with_credentials(true)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let quote_repo = QuoteRepo::new(env.d1("piqqi").expect("valid d1 db"));
    let line_item_repo = LineItemRepo::new(env.d1("piqqi").expect("valid d1 db"));
    let quote_service = QuoteService::new(quote_repo, line_item_repo);

    if req.method() == Method::Options {
        return Response::empty()?.with_cors(&cors());
    }

    Router::with_data(App { quote_service })
        .post_async("/quotes", quotes::api::create)
        .get_async("/quotes", quotes::api::list)
        .run(req.clone()?, env)
        .await?
        .with_cors(&cors())
}
