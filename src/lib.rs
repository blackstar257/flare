use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. The Router takes some data with its `new` method
    // that can be shared throughout all routes. If you don't need any shared data, use `()`.
    let router = Router::new(());

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and`
    // Enviornment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |req, _| {
            let f = req.headers().get("CF-Connecting-IP");
            let f = match f {
                Ok(ip) => ip,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            Response::ok(f.unwrap_or_default())
        })
        .run(req, env)
        .await
}
