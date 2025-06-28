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
    content_type: Option<String>,
    is_public: bool,
    food_orange: bool,
    food_apple: bool,
    food_banana: bool,
    food_melon: bool,
    food_grape: bool,
    pub_date1: Option<String>,
    pub_date2: Option<String>,
    pub_date3: Option<String>,
    pub_date4: Option<String>,
    pub_date5: Option<String>,
    pub_date6: Option<String>,
    qty1: Option<String>,
    qty2: Option<String>,
    qty3: Option<String>,
    qty4: Option<String>,
    qty5: Option<String>,
    qty6: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
    content_type: Option<String>,
    is_public: Option<bool>,
    food_orange: Option<bool>,
    food_apple: Option<bool>,
    food_banana: Option<bool>,
    food_melon: Option<bool>,
    food_grape: Option<bool>,
    pub_date1: Option<String>,
    pub_date2: Option<String>,
    pub_date3: Option<String>,
    pub_date4: Option<String>,
    pub_date5: Option<String>,
    pub_date6: Option<String>,
    qty1: Option<String>,
    qty2: Option<String>,
    qty3: Option<String>,
    qty4: Option<String>,
    qty5: Option<String>,
    qty6: Option<String>,
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
    content_type: Option<String>,
    is_public: Option<bool>,
    food_orange: Option<bool>,
    food_apple: Option<bool>,
    food_banana: Option<bool>,
    food_melon: Option<bool>,
    food_grape: Option<bool>,
    pub_date1: Option<String>,
    pub_date2: Option<String>,
    pub_date3: Option<String>,
    pub_date4: Option<String>,
    pub_date5: Option<String>,
    pub_date6: Option<String>,
    qty1: Option<String>,
    qty2: Option<String>,
    qty3: Option<String>,
    qty4: Option<String>,
    qty5: Option<String>,
    qty6: Option<String>,
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

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    //.post_async("/bar", handle_post)
    //        .delete_async("/baz", handle_delete)
    Router::new()
        .get_async("/api/list", handle_list_todos)
        .post_async("/api/create", handle_create_todo)
        .post_async("/api/delete", handle_delete_todo)
        .post_async("/api/update", handle_update_todo)
        .get_async("/foo", handle_get)
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

pub async fn handle_list_todos(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let query = "SELECT id, title, description, completed, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6, created_at, updated_at FROM todos ORDER BY created_at DESC";
    let stmt = db.prepare(query);
    let result = stmt.all().await?;
    
    let mut todos = Vec::new();
    
    if let Ok(results) = result.results::<serde_json::Value>() {
        for row in results {
            let todo = Todo {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                title: row["title"].as_str().unwrap_or_default().to_string(),
                description: row["description"].as_str().map(|s| s.to_string()),
                completed: row["completed"].as_f64().unwrap_or(0.0) != 0.0,
                content_type: row["content_type"].as_str().map(|s| s.to_string()),
                is_public: row["is_public"].as_f64().unwrap_or(0.0) != 0.0,
                food_orange: row["food_orange"].as_f64().unwrap_or(0.0) != 0.0,
                food_apple: row["food_apple"].as_f64().unwrap_or(0.0) != 0.0,
                food_banana: row["food_banana"].as_f64().unwrap_or(0.0) != 0.0,
                food_melon: row["food_melon"].as_f64().unwrap_or(0.0) != 0.0,
                food_grape: row["food_grape"].as_f64().unwrap_or(0.0) != 0.0,
                pub_date1: row["pub_date1"].as_str().map(|s| s.to_string()),
                pub_date2: row["pub_date2"].as_str().map(|s| s.to_string()),
                pub_date3: row["pub_date3"].as_str().map(|s| s.to_string()),
                pub_date4: row["pub_date4"].as_str().map(|s| s.to_string()),
                pub_date5: row["pub_date5"].as_str().map(|s| s.to_string()),
                pub_date6: row["pub_date6"].as_str().map(|s| s.to_string()),
                qty1: row["qty1"].as_str().map(|s| s.to_string()),
                qty2: row["qty2"].as_str().map(|s| s.to_string()),
                qty3: row["qty3"].as_str().map(|s| s.to_string()),
                qty4: row["qty4"].as_str().map(|s| s.to_string()),
                qty5: row["qty5"].as_str().map(|s| s.to_string()),
                qty6: row["qty6"].as_str().map(|s| s.to_string()),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            todos.push(todo);
        }
    }
    
    Response::from_json(&TodoListResponse {
        status: 200,
        data: todos,
    })
}

pub async fn handle_create_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let create_req: CreateTodoRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;
    
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
    
    let query = "INSERT INTO todos (title, description, completed, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6, created_at, updated_at) VALUES (?, ?, 0, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id, title, description, completed, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6, created_at, updated_at";
    let stmt = db.prepare(query)
        .bind(&[create_req.title.clone().into(), 
               create_req.description.clone().unwrap_or_default().into(),
               create_req.content_type.clone().unwrap_or_default().into(),
               (if create_req.is_public.unwrap_or(false) { 1 } else { 0 }).into(),
               (if create_req.food_orange.unwrap_or(false) { 1 } else { 0 }).into(),
               (if create_req.food_apple.unwrap_or(false) { 1 } else { 0 }).into(),
               (if create_req.food_banana.unwrap_or(false) { 1 } else { 0 }).into(),
               (if create_req.food_melon.unwrap_or(false) { 1 } else { 0 }).into(),
               (if create_req.food_grape.unwrap_or(false) { 1 } else { 0 }).into(),
               create_req.pub_date1.clone().unwrap_or_default().into(),
               create_req.pub_date2.clone().unwrap_or_default().into(),
               create_req.pub_date3.clone().unwrap_or_default().into(),
               create_req.pub_date4.clone().unwrap_or_default().into(),
               create_req.pub_date5.clone().unwrap_or_default().into(),
               create_req.pub_date6.clone().unwrap_or_default().into(),
               create_req.qty1.clone().unwrap_or_default().into(),
               create_req.qty2.clone().unwrap_or_default().into(),
               create_req.qty3.clone().unwrap_or_default().into(),
               create_req.qty4.clone().unwrap_or_default().into(),
               create_req.qty5.clone().unwrap_or_default().into(),
               create_req.qty6.clone().unwrap_or_default().into(),
               now.clone().into(),
               now.clone().into()])?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = Todo {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                title: row["title"].as_str().unwrap_or_default().to_string(),
                description: row["description"].as_str().map(|s| s.to_string()),
                completed: row["completed"].as_f64().unwrap_or(0.0) != 0.0,
                content_type: row["content_type"].as_str().map(|s| s.to_string()),
                is_public: row["is_public"].as_f64().unwrap_or(0.0) != 0.0,
                food_orange: row["food_orange"].as_f64().unwrap_or(0.0) != 0.0,
                food_apple: row["food_apple"].as_f64().unwrap_or(0.0) != 0.0,
                food_banana: row["food_banana"].as_f64().unwrap_or(0.0) != 0.0,
                food_melon: row["food_melon"].as_f64().unwrap_or(0.0) != 0.0,
                food_grape: row["food_grape"].as_f64().unwrap_or(0.0) != 0.0,
                pub_date1: row["pub_date1"].as_str().map(|s| s.to_string()),
                pub_date2: row["pub_date2"].as_str().map(|s| s.to_string()),
                pub_date3: row["pub_date3"].as_str().map(|s| s.to_string()),
                pub_date4: row["pub_date4"].as_str().map(|s| s.to_string()),
                pub_date5: row["pub_date5"].as_str().map(|s| s.to_string()),
                pub_date6: row["pub_date6"].as_str().map(|s| s.to_string()),
                qty1: row["qty1"].as_str().map(|s| s.to_string()),
                qty2: row["qty2"].as_str().map(|s| s.to_string()),
                qty3: row["qty3"].as_str().map(|s| s.to_string()),
                qty4: row["qty4"].as_str().map(|s| s.to_string()),
                qty5: row["qty5"].as_str().map(|s| s.to_string()),
                qty6: row["qty6"].as_str().map(|s| s.to_string()),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            
            Response::from_json(&TodoCreateResponse {
                status: 201,
                data: todo,
            })
        },
        None => {
            Response::from_json(&GenericResponse {
                status: 500,
                message: "Failed to create todo".to_string(),
            })
        }
    }
}

pub async fn handle_delete_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let delete_req: DeleteTodoRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;
    
    let query = "DELETE FROM todos WHERE id = ?";
    let stmt = db.prepare(query).bind(&[delete_req.id.into()])?;
    
    let result = stmt.run().await?;
    Response::from_json(&GenericResponse {
        status: 200,
        message: "Todo deleted successfully".to_string(),
    })
}

pub async fn handle_update_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let update_req: UpdateTodoRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;
    
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
    
    // First check if the todo exists
    let check_query = "SELECT id FROM todos WHERE id = ?";
    let check_stmt = db.prepare(check_query).bind(&[update_req.id.into()])?;
    let exists = check_stmt.first::<serde_json::Value>(None).await?;
    
    if exists.is_none() {
        return Response::from_json(&GenericResponse {
            status: 404,
            message: "Todo not found".to_string(),
        });
    }
    
    // Build dynamic update query
    let mut query_parts = Vec::new();
    let mut bindings = Vec::new();
    
    if let Some(title) = &update_req.title {
        query_parts.push("title = ?");
        bindings.push(title.clone().into());
    }
    
    if let Some(description) = &update_req.description {
        query_parts.push("description = ?");
        bindings.push(description.clone().into());
    }
    
    if let Some(completed) = update_req.completed {
        query_parts.push("completed = ?");
        bindings.push((if completed { 1 } else { 0 }).into());
    }
    
    if let Some(content_type) = &update_req.content_type {
        query_parts.push("content_type = ?");
        bindings.push(content_type.clone().into());
    }
    
    if let Some(is_public) = update_req.is_public {
        query_parts.push("is_public = ?");
        bindings.push((if is_public { 1 } else { 0 }).into());
    }
    
    if let Some(food_orange) = update_req.food_orange {
        query_parts.push("food_orange = ?");
        bindings.push((if food_orange { 1 } else { 0 }).into());
    }
    
    if let Some(food_apple) = update_req.food_apple {
        query_parts.push("food_apple = ?");
        bindings.push((if food_apple { 1 } else { 0 }).into());
    }
    
    if let Some(food_banana) = update_req.food_banana {
        query_parts.push("food_banana = ?");
        bindings.push((if food_banana { 1 } else { 0 }).into());
    }
    
    if let Some(food_melon) = update_req.food_melon {
        query_parts.push("food_melon = ?");
        bindings.push((if food_melon { 1 } else { 0 }).into());
    }
    
    if let Some(food_grape) = update_req.food_grape {
        query_parts.push("food_grape = ?");
        bindings.push((if food_grape { 1 } else { 0 }).into());
    }
    
    if let Some(pub_date1) = &update_req.pub_date1 {
        query_parts.push("pub_date1 = ?");
        bindings.push(pub_date1.clone().into());
    }
    
    if let Some(pub_date2) = &update_req.pub_date2 {
        query_parts.push("pub_date2 = ?");
        bindings.push(pub_date2.clone().into());
    }
    
    if let Some(pub_date3) = &update_req.pub_date3 {
        query_parts.push("pub_date3 = ?");
        bindings.push(pub_date3.clone().into());
    }
    
    if let Some(pub_date4) = &update_req.pub_date4 {
        query_parts.push("pub_date4 = ?");
        bindings.push(pub_date4.clone().into());
    }
    
    if let Some(pub_date5) = &update_req.pub_date5 {
        query_parts.push("pub_date5 = ?");
        bindings.push(pub_date5.clone().into());
    }
    
    if let Some(pub_date6) = &update_req.pub_date6 {
        query_parts.push("pub_date6 = ?");
        bindings.push(pub_date6.clone().into());
    }
    
    if let Some(qty1) = &update_req.qty1 {
        query_parts.push("qty1 = ?");
        bindings.push(qty1.clone().into());
    }
    
    if let Some(qty2) = &update_req.qty2 {
        query_parts.push("qty2 = ?");
        bindings.push(qty2.clone().into());
    }
    
    if let Some(qty3) = &update_req.qty3 {
        query_parts.push("qty3 = ?");
        bindings.push(qty3.clone().into());
    }
    
    if let Some(qty4) = &update_req.qty4 {
        query_parts.push("qty4 = ?");
        bindings.push(qty4.clone().into());
    }
    
    if let Some(qty5) = &update_req.qty5 {
        query_parts.push("qty5 = ?");
        bindings.push(qty5.clone().into());
    }
    
    if let Some(qty6) = &update_req.qty6 {
        query_parts.push("qty6 = ?");
        bindings.push(qty6.clone().into());
    }
    
    if query_parts.is_empty() {
        return Response::from_json(&GenericResponse {
            status: 400,
            message: "No fields to update".to_string(),
        });
    }
    
    query_parts.push("updated_at = ?");
    bindings.push(now.into());
    bindings.push(update_req.id.into());
    
    let query = format!("UPDATE todos SET {} WHERE id = ? RETURNING id, title, description, completed, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6, created_at, updated_at", query_parts.join(", "));
    let stmt = db.prepare(&query).bind(&bindings)?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = Todo {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                title: row["title"].as_str().unwrap_or_default().to_string(),
                description: row["description"].as_str().map(|s| s.to_string()),
                completed: row["completed"].as_f64().unwrap_or(0.0) != 0.0,
                content_type: row["content_type"].as_str().map(|s| s.to_string()),
                is_public: row["is_public"].as_f64().unwrap_or(0.0) != 0.0,
                food_orange: row["food_orange"].as_f64().unwrap_or(0.0) != 0.0,
                food_apple: row["food_apple"].as_f64().unwrap_or(0.0) != 0.0,
                food_banana: row["food_banana"].as_f64().unwrap_or(0.0) != 0.0,
                food_melon: row["food_melon"].as_f64().unwrap_or(0.0) != 0.0,
                food_grape: row["food_grape"].as_f64().unwrap_or(0.0) != 0.0,
                pub_date1: row["pub_date1"].as_str().map(|s| s.to_string()),
                pub_date2: row["pub_date2"].as_str().map(|s| s.to_string()),
                pub_date3: row["pub_date3"].as_str().map(|s| s.to_string()),
                pub_date4: row["pub_date4"].as_str().map(|s| s.to_string()),
                pub_date5: row["pub_date5"].as_str().map(|s| s.to_string()),
                pub_date6: row["pub_date6"].as_str().map(|s| s.to_string()),
                qty1: row["qty1"].as_str().map(|s| s.to_string()),
                qty2: row["qty2"].as_str().map(|s| s.to_string()),
                qty3: row["qty3"].as_str().map(|s| s.to_string()),
                qty4: row["qty4"].as_str().map(|s| s.to_string()),
                qty5: row["qty5"].as_str().map(|s| s.to_string()),
                qty6: row["qty6"].as_str().map(|s| s.to_string()),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            
            Response::from_json(&TodoCreateResponse {
                status: 200,
                data: todo,
            })
        },
        None => {
            Response::from_json(&GenericResponse {
                status: 500,
                message: "Failed to update todo".to_string(),
            })
        }
    }
}