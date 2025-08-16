use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteTodoRequest {
    id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdateTodoRequest {
    id: u32,
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoListResponse {
    status: u16,
    data: Vec<Todo>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoCreateResponse {
    status: u16,
    data: Todo,
}
mod handlers; // handlers/mod.rs を指す

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    //todo_handlers , test_handlers
    Router::new()
        .get_async("/api/todos/list", handlers::test_handlers::handle_list_todos)
        .post_async("/api/todos/create", handlers::todo_handlers::handle_create_todo)
        .post_async("/api/todos/delete", handlers::todo_handlers::handle_delete_todo)
        .post_async("/api/todos/update", handlers::todo_handlers::handle_update_todo)
        .get_async("/api/todo13/list", handlers::todo13_handlers::handle_list_todos)
        .post_async("/api/todo13/create", handlers::todo13_handlers::handle_create_todo)
        .post_async("/api/todo13/delete", handlers::todo13_handlers::handle_delete_todo)
        .post_async("/api/todo13/update", handlers::todo13_handlers::handle_update_todo)
        .get_async("/api/todo16/list", handlers::todo16_handlers::handle_list_todos)
        .post_async("/api/todo16/create", handlers::todo16_handlers::handle_create_todo)
        .post_async("/api/todo16/delete", handlers::todo16_handlers::handle_delete_todo)
        .post_async("/api/todo16/update", handlers::todo16_handlers::handle_update_todo)

        .get_async("/", handle_get) 
        .get_async("/todo", handle_get) 
        .get_async("/todo13", handle_get) 
        .get_async("/todo16", handle_get) 
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
