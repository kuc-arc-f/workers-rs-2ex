use serde::{Deserialize, Serialize};
use worker::*;
//use console::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Plan {
    id: u32,
    user_id: Option<u32>,
    content: String,
    p_date: Option<String>,
    created_at: String,
    updated_at: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct RangePlanRequest {
    user_id: Option<u32>,
    start: String,
    end: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct CreatePlanRequest {
    user_id: Option<u32>,
    content: String,
    p_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeletePlanRequest {
    id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdatePlanRequest {
    id: u32,
    user_id: Option<u32>,
    content: Option<String>,
    p_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlanListResponse {
    status: u16,
    data: Vec<Plan>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlanResponse {
    status: u16,
    data: Plan,
}

pub async fn handle_list_range(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    let range_req: RangePlanRequest = req.json().await?;
    //console_log!("Request method = {}", req.method());
    console_log!("{:?}", range_req);

    let query = "SELECT id, user_id, content, p_date, created_at, updated_at 
    FROM plan 
    WHERE 
    (
    p_date >= datetime(?, 'localtime')
    AND p_date < datetime(? , 'localtime')
    )
    AND user_id = ?    
    ORDER BY created_at DESC";

    let stmt = db.prepare(query)
            .bind(&[
            range_req.start.clone().into(),
            range_req.end.clone().into(),
            range_req.user_id.into()
            ])?;
               
    let result = stmt.all().await?;

    let mut plans = Vec::new();

    if let Ok(results) = result.results::<serde_json::Value>() {
        for row in results {
            let plan = Plan {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                user_id: row["user_id"].as_f64().map(|v| v as u32),
                content: row["content"].as_str().unwrap_or_default().to_string(),
                p_date: row["p_date"].as_str().map(String::from),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            plans.push(plan);
        }
    }

    Response::from_json(&PlanListResponse {
        status: 200,
        data: plans,
    })
}

pub async fn handle_list_plans(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;

    let query = "SELECT id, user_id, content, p_date, created_at, updated_at FROM plan ORDER BY created_at DESC";
    let stmt = db.prepare(query);
    let result = stmt.all().await?;

    let mut plans = Vec::new();

    if let Ok(results) = result.results::<serde_json::Value>() {
        for row in results {
            let plan = Plan {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                user_id: row["user_id"].as_f64().map(|v| v as u32),
                content: row["content"].as_str().unwrap_or_default().to_string(),
                p_date: row["p_date"].as_str().map(String::from),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            plans.push(plan);
        }
    }

    Response::from_json(&PlanListResponse {
        status: 200,
        data: plans,
    })
}

pub async fn handle_create_plan(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let create_req: CreatePlanRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;

    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();

    let query = "INSERT INTO plan 
    (user_id, content, p_date, created_at, updated_at
    ) VALUES (
     ?, ?, datetime(?, 'localtime'), ?, ?) 
    RETURNING id, user_id, content, p_date, created_at, updated_at";
    let stmt = db.prepare(query)
        .bind(&[
            //create_req.user_id.map(|v| v.into()).unwrap_or(JsValue::NULL),
            create_req.user_id.into(),
            create_req.content.into(),
            create_req.p_date.into(),
            now.clone().into(),
            now.clone().into(),
        ])?;

    let result = stmt.first::<serde_json::Value>(None).await?;

    match result {
        Some(row) => {
            let plan = Plan {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                user_id: row["user_id"].as_f64().map(|v| v as u32),
                content: row["content"].as_str().unwrap_or_default().to_string(),
                p_date: row["p_date"].as_str().map(String::from),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };

            Response::from_json(&PlanResponse {
                status: 201,
                data: plan,
            })
        },
        None => {
            Response::from_json(&GenericResponse {
                status: 500,
                message: "Failed to create plan".to_string(),
            })
        }
    }
}

pub async fn handle_delete_plan(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let delete_req: DeletePlanRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;

    let query = "DELETE FROM plan WHERE id = ?";
    let stmt = db.prepare(query).bind(&[delete_req.id.into()])?;

    stmt.run().await?;
    Response::from_json(&GenericResponse {
        status: 200,
        message: "Plan deleted successfully".to_string(),
    })
}

pub async fn handle_update_plan(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let update_req: UpdatePlanRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;

    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();

    // First check if the plan exists
    let check_query = "SELECT id FROM plan WHERE id = ?";
    let check_stmt = db.prepare(check_query).bind(&[update_req.id.into()])?;
    let exists = check_stmt.first::<serde_json::Value>(None).await?;

    if exists.is_none() {
        return Response::from_json(&GenericResponse {
            status: 404,
            message: "Plan not found".to_string(),
        });
    }

    // Build dynamic update query
    let mut query_parts = Vec::new();
    let mut bindings = Vec::new();

    if let Some(user_id) = update_req.user_id {
        query_parts.push("user_id = ?");
        bindings.push(user_id.into());
    }
    if let Some(content) = &update_req.content {
        query_parts.push("content = ?");
        bindings.push(content.clone().into());
    }
    //if let Some(p_date) = &update_req.p_date {
    //    query_parts.push("p_date = ?");
    //    bindings.push(p_date.clone().into());
    //}

    if query_parts.is_empty() {
        return Response::from_json(&GenericResponse {
            status: 400,
            message: "No fields to update".to_string(),
        });
    }

    query_parts.push("updated_at = ?");
    bindings.push(now.into());
    bindings.push(update_req.id.into());

    let query = format!("UPDATE plan SET {} WHERE id = ? RETURNING id, user_id, content, p_date, created_at, updated_at", query_parts.join(", "));
    let stmt = db.prepare(&query).bind(&bindings)?;

    let result = stmt.first::<serde_json::Value>(None).await?;

    match result {
        Some(row) => {
            let plan = Plan {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                user_id: row["user_id"].as_f64().map(|v| v as u32),
                content: row["content"].as_str().unwrap_or_default().to_string(),
                p_date: row["p_date"].as_str().map(String::from),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };

            Response::from_json(&PlanResponse {
                status: 200,
                data: plan,
            })
        },
        None => {
            Response::from_json(&GenericResponse {
                status: 500,
                message: "Failed to update plan".to_string(),
            })
        }
    }
}