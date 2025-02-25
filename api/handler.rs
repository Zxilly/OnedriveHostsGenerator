use onedrive_hosts_generator::render;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let ipv4 = hash_query.contains_key("ipv4");
    let ipv6 = hash_query.contains_key("ipv6");
    let single = hash_query.contains_key("single");

    let mut ret = if !ipv4 && !ipv6 {
        render(true, true, single)
    } else {
        render(ipv4, ipv6, single)
    }
    .await;

    if url
        .path_segments()
        .is_some_and(|mut segs| segs.next() == Some("dns.cache"))
    {
        const WARN: &str =
            "# dns.cache is a deprecated endpoint from old php version, please use / instead.\n";
        ret = format!("{}{}", WARN, ret);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .header(
            "Cache-Control",
            "max-age=36000, s-maxage=36000, stale-while-revalidate=100, public",
        )
        .header("CDN-Cache-Control", "max-age=36000, public")
        .header("Vercel-CDN-Cache-Control", "max-age=36000, public")
        .body(Body::Text(ret))?)
}
