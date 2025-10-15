use worker::*;

pub mod resources;
pub mod server;

use crate::resources::quotes::{self, repository::QuoteRepo};

pub struct App {
    quote_repo: QuoteRepo,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let db = env
        .d1("piqqi")
        .map_err(|_e| worker::Error::RustError("D1 binding not found".to_string()))?;

    let quote_repo = QuoteRepo::new(db);

    Router::with_data(App { quote_repo })
        .post_async("/quotes", quotes::api::create)
        // .get_async("/quotes", quotes::api::list)
        // .get_async("/quotes/:id", quotes::api::get)
        // .put_async("/quotes/:id", quotes::api::update)
        // .delete_async("/quotes/:id", quotes::api::delete)
        .run(req, env)
        .await
}
