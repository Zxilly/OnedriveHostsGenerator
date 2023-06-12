use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use onedrive_hosts_generator::render;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let ret = render();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::Text(ret))?)
}