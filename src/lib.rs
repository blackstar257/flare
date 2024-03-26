use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new().get_async("/", handle_get).run(req, env).await
}

pub async fn handle_get(req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let ip_header = req.headers().get("CF-Connecting-IP");
    let ip = match ip_header {
        Ok(ip) => ip,
        Err(_) => None,
    };
    let message = ip.unwrap_or("unknown".to_owned()).to_string();
    Response::from_json(&GenericResponse {
        status: 200,
        message: message,
    })
}
