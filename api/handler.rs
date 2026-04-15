use http::StatusCode;
use onedrive_hosts_generator::render;
use std::collections::HashMap;
use url::form_urlencoded;
use vercel_runtime::{run, service_fn, Error, Request, Response, ResponseBody};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

pub async fn handler(req: Request) -> Result<Response<ResponseBody>, Error> {
    let uri = req.uri();
    let hash_query: HashMap<String, String> = uri
        .query()
        .map(|q| form_urlencoded::parse(q.as_bytes()).into_owned().collect())
        .unwrap_or_default();

    let ipv4 = hash_query.contains_key("ipv4");
    let ipv6 = hash_query.contains_key("ipv6");
    let single = hash_query.contains_key("single");

    let ret = if !ipv4 && !ipv6 {
        render(true, true, single)
    } else {
        render(ipv4, ipv6, single)
    }
    .await;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .header(
            "Cache-Control",
            "max-age=36000, s-maxage=36000, stale-while-revalidate=100, public",
        )
        .header("CDN-Cache-Control", "max-age=36000, public")
        .header("Vercel-CDN-Cache-Control", "max-age=36000, public")
        .body(ret.into())?)
}
