use vercel_runtime::{run, Body, Error, Request, Response, StatusCode, RequestExt};
use onedrive_hosts_generator::render;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query = req.query_string_parameters();
    let ipv4 = query.all("ipv4").is_some();
    let ipv6 = query.all("ipv6").is_some();

    let ret = if !ipv4 && !ipv6 {
        render(true, true)
    } else {
        render(ipv4, ipv6)
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .header("Cache-Control", "s-maxage=6000, stale-while-revalidate=10")
        .body(Body::Text(ret))?)
}