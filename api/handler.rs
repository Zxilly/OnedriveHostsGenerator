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

    let ipv4 = hash_query.get("ipv4").is_some();
    let ipv6 = hash_query.get("ipv6").is_some();
    let single = hash_query.get("single").is_some();

    let (mut ret, ttl) = if !ipv4 && !ipv6 {
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
            format!(
                "max-age={}, s-maxage={}, stale-while-revalidate=10, public",
                ttl, ttl
            ),
        )
        .body(Body::Text(ret))?)
}
