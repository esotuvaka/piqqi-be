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

fn cors_response(req: Request) -> Result<Response> {
    let mut res = Response::empty()?;
    let headers = res.headers_mut();
    if let Some(origin) = req.headers().get("Origin")? {
        headers.set("Access-Control-Allow-Origin", &origin)?;
    } else {
        headers.set("Access-Control-Allow-Origin", "*")?;
    }

    headers.set("Vary", "Origin")?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    )?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization",
    )?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(res)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let quote_repo = QuoteRepo::new(env.d1("piqqi").expect("valid d1 db"));
    let line_item_repo = LineItemRepo::new(env.d1("piqqi").expect("valid d1 db"));
    let quote_service = QuoteService::new(quote_repo, line_item_repo);

    if req.method() == Method::Options {
        return cors_response(req);
    }

    Router::with_data(App { quote_service })
        .post_async("/quotes", quotes::api::create)
        .get_async("/quotes", quotes::api::list)
        // .get_async("/quotes/:id", quotes::api::get)
        // .put_async("/quotes/:id", quotes::api::update)
        // .delete_async("/quotes/:id", quotes::api::delete)
        .run(req, env)
        .await
}
