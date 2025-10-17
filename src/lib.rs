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

fn with_cors(req: &Request, mut res: Response) -> Result<Response> {
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
        let res = Response::empty()?;
        return with_cors(&req, res);
    }

    let router = Router::with_data(App { quote_service })
        .post_async("/quotes", quotes::api::create)
        .get_async("/quotes", quotes::api::list);

    let res = router.run(req.clone()?, env).await?;

    with_cors(&req, res)
}
