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


pub async fn handle_list_todos(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let query = "SELECT id, title, description, completed, created_at, updated_at FROM todos ORDER BY created_at DESC";
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
    console_log!("{:?}", create_req);
    
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
    
    let query = "INSERT INTO todos (title, description, completed, created_at, updated_at) VALUES (?, ?, 0, ?, ?) RETURNING id, title, description, completed, created_at, updated_at";
    let stmt = db.prepare(query)
        .bind(&[create_req.title.clone().into(), 
               create_req.description.clone().unwrap_or_default().into(),
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
    
    if query_parts.is_empty() {
        return Response::from_json(&GenericResponse {
            status: 400,
            message: "No fields to update".to_string(),
        });
    }
    
    query_parts.push("updated_at = ?");
    bindings.push(now.into());
    bindings.push(update_req.id.into());
    
    let query = format!("UPDATE todos SET {} WHERE id = ? RETURNING id, title, description, completed, created_at, updated_at", query_parts.join(", "));
    let stmt = db.prepare(&query).bind(&bindings)?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = Todo {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                title: row["title"].as_str().unwrap_or_default().to_string(),
                description: row["description"].as_str().map(|s| s.to_string()),
                completed: row["completed"].as_f64().unwrap_or(0.0) != 0.0,
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
