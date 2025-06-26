use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/foo", handle_get)
        .post_async("/bar", handle_post)
        .delete_async("/baz", handle_delete)
        .run(req, env)
        .await
}

pub async fn handle_get(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    // HTML文字列を生成  
    let html = format!(
      "<!DOCTYPE html>
      <html>
        <head>
          <meta charset=\"utf-8\">
          <meta name='viewport' content='width=device-width, initial-scale=1.0' />
          <title>My Page</title>
          <script src='https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4'></script>
        </head>
        <body>
          <div id='app'></div>
          <script type='module' src='/client.js'></script>
        </body>
      </html>"
    );
    // Response::from_html を使うとヘッダー付きで便利
    let mut response = Response::from_html(html)?;
    // 必要に応じてヘッダーをカスタム可能
    response.headers_mut()
        .set("Content-Type", "text/html; charset=utf-8")?;
    Ok(response)
}

pub async fn handle_post(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a POST route!".to_string(),
    })
}

pub async fn handle_delete(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a DELETE route!".to_string(),
    })
}
