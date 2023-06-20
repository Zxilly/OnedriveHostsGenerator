use std::collections::HashMap;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use onedrive_hosts_generator::render;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = query.query_pairs().into_owned().collect();

    let ipv4 = hash_query.get("ipv4").is_some();
    let ipv6 = hash_query.get("ipv6").is_some();

    let ret = if !ipv4 && !ipv6 {
        render(true, true)
    } else {
        render(ipv4, ipv6)
    }.await;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .header("Cache-Control", "s-maxage=6000, stale-while-revalidate=10")
        .body(Body::Text(ret))?)
}