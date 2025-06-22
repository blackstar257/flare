use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", handle_get)
        .get_async("/ip", handle_get)
        .get_async("/debug", handle_debug)
        .get_async("/time", handle_time)
        .options_async("/*", handle_options)
        .run(req, env)
        .await
}

// Helper function to add CORS headers to a response
fn add_cors_headers(mut response: Response) -> worker::Result<Response> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, DELETE, OPTIONS",
    )?;
    headers.set("Access-Control-Allow-Headers", "*")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(response)
}

pub async fn handle_get(req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let ip_header = req.headers().get("CF-Connecting-IP");
    let ip = match ip_header {
        Ok(ip) => ip,
        Err(_) => None,
    };
    let message = ip.unwrap_or("unknown".to_owned()).to_string();
    let response = Response::ok(message)?;
    add_cors_headers(response)
}

pub async fn handle_debug(req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let headers = req.headers();
    let mut headers_map = std::collections::HashMap::new();

    for (name, value) in headers.entries() {
        headers_map.insert(name, value);
    }

    let response = Response::from_json(&headers_map)?;
    add_cors_headers(response)
}

pub async fn handle_time(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let now = js_sys::Date::new_0();
    let datetime_string = now.to_iso_string();
    let response = Response::ok(
        datetime_string
            .as_string()
            .unwrap_or("Error getting time".to_string()),
    )?;
    add_cors_headers(response)
}

pub async fn handle_options(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let response = Response::empty()?;
    add_cors_headers(response)
}
