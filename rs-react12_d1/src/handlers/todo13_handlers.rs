use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    content: Option<String>,
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
    content: Option<String>,
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
    content: Option<String>,
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

fn json_to_todo(row: &serde_json::Value) -> Todo {
    Todo {
        id: row["id"].as_f64().unwrap_or(0.0) as u32,
        title: row["title"].as_str().unwrap_or_default().to_string(),
        content: row["content"].as_str().map(String::from),
        content_type: row["content_type"].as_str().map(String::from),
        is_public: row["is_public"].as_i64().unwrap_or(0) == 1,
        food_orange: row["food_orange"].as_i64().unwrap_or(0) == 1,
        food_apple: row["food_apple"].as_i64().unwrap_or(0) == 1,
        food_banana: row["food_banana"].as_i64().unwrap_or(0) == 1,
        food_melon: row["food_melon"].as_i64().unwrap_or(0) == 1,
        food_grape: row["food_grape"].as_i64().unwrap_or(0) == 1,
        pub_date1: row["pub_date1"].as_str().map(String::from),
        pub_date2: row["pub_date2"].as_str().map(String::from),
        pub_date3: row["pub_date3"].as_str().map(String::from),
        pub_date4: row["pub_date4"].as_str().map(String::from),
        pub_date5: row["pub_date5"].as_str().map(String::from),
        pub_date6: row["pub_date6"].as_str().map(String::from),
        qty1: row["qty1"].as_str().map(String::from),
        qty2: row["qty2"].as_str().map(String::from),
        qty3: row["qty3"].as_str().map(String::from),
        qty4: row["qty4"].as_str().map(String::from),
        qty5: row["qty5"].as_str().map(String::from),
        qty6: row["qty6"].as_str().map(String::from),
        created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
        updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
    }
}

pub async fn handle_list_todos(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let query = "SELECT * FROM todo13 ORDER BY created_at DESC";
    let stmt = db.prepare(query);
    let result = stmt.all().await?;
    
    let mut todos = Vec::new();
    
    if let Ok(results) = result.results::<serde_json::Value>() {
        for row in results {
            todos.push(json_to_todo(&row));
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
    
    let query = "INSERT INTO todo13 (title, content, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *";
    let stmt = db.prepare(query)
        .bind(&[
            create_req.title.into(),
            create_req.content.into(),
            create_req.content_type.into(),
            create_req.is_public.unwrap_or(false).into(),
            create_req.food_orange.unwrap_or(false).into(),
            create_req.food_apple.unwrap_or(false).into(),
            create_req.food_banana.unwrap_or(false).into(),
            create_req.food_melon.unwrap_or(false).into(),
            create_req.food_grape.unwrap_or(false).into(),
            create_req.pub_date1.into(),
            create_req.pub_date2.into(),
            create_req.pub_date3.into(),
            create_req.pub_date4.into(),
            create_req.pub_date5.into(),
            create_req.pub_date6.into(),
            create_req.qty1.into(),
            create_req.qty2.into(),
            create_req.qty3.into(),
            create_req.qty4.into(),
            create_req.qty5.into(),
            create_req.qty6.into(),
            now.clone().into(),
            now.clone().into(),
        ])?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = json_to_todo(&row);
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
    
    let query = "DELETE FROM todo13 WHERE id = ?";
    let stmt = db.prepare(query).bind(&[delete_req.id.into()])?;
    
    stmt.run().await?;

    Response::from_json(&GenericResponse {
        status: 200,
        message: "Todo deleted successfully".to_string(),
    })
}

pub async fn handle_update_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let update_req: UpdateTodoRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;
    
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
    
    let check_query = "SELECT id FROM todo13 WHERE id = ?";
    let check_stmt = db.prepare(check_query).bind(&[update_req.id.into()])?;
    if check_stmt.first::<serde_json::Value>(None).await?.is_none() {
        return Response::from_json(&GenericResponse {
            status: 404,
            message: "Todo not found".to_string(),
        });
    }
    
    let mut query_parts = Vec::new();
    let mut bindings = Vec::new();
    
    if let Some(title) = update_req.title {
        query_parts.push("title = ?");
        bindings.push(title.into());
    }
    if let Some(content) = update_req.content {
        query_parts.push("content = ?");
        bindings.push(content.into());
    }
    if let Some(content_type) = update_req.content_type {
        query_parts.push("content_type = ?");
        bindings.push(content_type.into());
    }
    if let Some(is_public) = update_req.is_public {
        query_parts.push("is_public = ?");
        bindings.push(is_public.into());
    }
    if let Some(food_orange) = update_req.food_orange {
        query_parts.push("food_orange = ?");
        bindings.push(food_orange.into());
    }
    if let Some(food_apple) = update_req.food_apple {
        query_parts.push("food_apple = ?");
        bindings.push(food_apple.into());
    }
    if let Some(food_banana) = update_req.food_banana {
        query_parts.push("food_banana = ?");
        bindings.push(food_banana.into());
    }
    if let Some(food_melon) = update_req.food_melon {
        query_parts.push("food_melon = ?");
        bindings.push(food_melon.into());
    }
    if let Some(food_grape) = update_req.food_grape {
        query_parts.push("food_grape = ?");
        bindings.push(food_grape.into());
    }
    if let Some(pub_date1) = update_req.pub_date1 {
        query_parts.push("pub_date1 = ?");
        bindings.push(pub_date1.into());
    }
    if let Some(pub_date2) = update_req.pub_date2 {
        query_parts.push("pub_date2 = ?");
        bindings.push(pub_date2.into());
    }
    if let Some(pub_date3) = update_req.pub_date3 {
        query_parts.push("pub_date3 = ?");
        bindings.push(pub_date3.into());
    }
    if let Some(pub_date4) = update_req.pub_date4 {
        query_parts.push("pub_date4 = ?");
        bindings.push(pub_date4.into());
    }
    if let Some(pub_date5) = update_req.pub_date5 {
        query_parts.push("pub_date5 = ?");
        bindings.push(pub_date5.into());
    }
    if let Some(pub_date6) = update_req.pub_date6 {
        query_parts.push("pub_date6 = ?");
        bindings.push(pub_date6.into());
    }
    if let Some(qty1) = update_req.qty1 {
        query_parts.push("qty1 = ?");
        bindings.push(qty1.into());
    }
        if let Some(qty2) = update_req.qty2 {
        query_parts.push("qty2 = ?");
        bindings.push(qty2.into());
    }
    if let Some(qty3) = update_req.qty3 {
        query_parts.push("qty3 = ?");
        bindings.push(qty3.into());
    }
    if let Some(qty4) = update_req.qty4 {
        query_parts.push("qty4 = ?");
        bindings.push(qty4.into());
    }
    if let Some(qty5) = update_req.qty5 {
        query_parts.push("qty5 = ?");
        bindings.push(qty5.into());
    }
    if let Some(qty6) = update_req.qty6 {
        query_parts.push("qty6 = ?");
        bindings.push(qty6.into());
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
    
    let query = format!("UPDATE todo13 SET {} WHERE id = ? RETURNING *", query_parts.join(", "));
    let stmt = db.prepare(&query).bind(&bindings)?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = json_to_todo(&row);
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