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
    title: Option<String>,
    content: Option<String>,
    content_type: Option<String>,
    public_type: Option<String>,
    food_orange: bool,
    food_apple: bool,
    food_banana: bool,
    food_melon: bool,
    food_grape: bool,
    category_food: bool,
    category_drink: bool,
    category_gadget: bool,
    category_sport: bool,
    category_government: bool,
    category_internet: bool,
    category_smartphone: bool,
    country_jp: Option<String>,
    country_en: Option<String>,
    prefecture_jp: Option<String>,
    prefecture_en: Option<String>,
    post_no_jp: Option<String>,
    post_no_en: Option<String>,
    address_1_jp: Option<String>,
    address_1_en: Option<String>,
    address_2_jp: Option<String>,
    address_2_en: Option<String>,
    address_other_jp: Option<String>,
    address_other_en: Option<String>,
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
    //created_at: String,
    //updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateTodoRequest {
    title: Option<String>,
    content: Option<String>,
    content_type: Option<String>,
    public_type: Option<String>,
    food_orange: bool,
    food_apple: bool,
    food_banana: bool,
    food_melon: bool,
    food_grape: bool,
    category_food: bool,
    category_drink: bool,
    category_gadget: bool,
    category_sport: bool,
    category_government: bool,
    category_internet: bool,
    category_smartphone: bool,
    country_jp: Option<String>,
    country_en: Option<String>,
    prefecture_jp: Option<String>,
    prefecture_en: Option<String>,
    post_no_jp: Option<String>,
    post_no_en: Option<String>,
    address_1_jp: Option<String>,
    address_1_en: Option<String>,
    address_2_jp: Option<String>,
    address_2_en: Option<String>,
    address_other_jp: Option<String>,
    address_other_en: Option<String>,
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
    public_type: Option<String>,
    food_orange: Option<bool>,
    food_apple: Option<bool>,
    food_banana: Option<bool>,
    food_melon: Option<bool>,
    food_grape: Option<bool>,
    category_food: Option<bool>,
    category_drink: Option<bool>,
    category_gadget: Option<bool>,
    category_sport: Option<bool>,
    category_government: Option<bool>,
    category_internet: Option<bool>,
    category_smartphone: Option<bool>,
    country_jp: Option<String>,
    country_en: Option<String>,
    prefecture_jp: Option<String>,
    prefecture_en: Option<String>,
    post_no_jp: Option<String>,
    post_no_en: Option<String>,
    address_1_jp: Option<String>,
    address_1_en: Option<String>,
    address_2_jp: Option<String>,
    address_2_en: Option<String>,
    address_other_jp: Option<String>,
    address_other_en: Option<String>,
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
        title: row["title"].as_str().map(String::from),
        content: row["content"].as_str().map(String::from),
        content_type: row["content_type"].as_str().map(String::from),
        public_type: row["public_type"].as_str().map(String::from),
        food_orange: row["food_orange"].as_bool().unwrap_or(false),
        food_apple: row["food_apple"].as_bool().unwrap_or(false),
        food_banana: row["food_banana"].as_bool().unwrap_or(false),
        food_melon: row["food_melon"].as_bool().unwrap_or(false),
        food_grape: row["food_grape"].as_bool().unwrap_or(false),
        category_food: row["category_food"].as_bool().unwrap_or(false),
        category_drink: row["category_drink"].as_bool().unwrap_or(false),
        category_gadget: row["category_gadget"].as_bool().unwrap_or(false),
        category_sport: row["category_sport"].as_bool().unwrap_or(false),
        category_government: row["category_government"].as_bool().unwrap_or(false),
        category_internet: row["category_internet"].as_bool().unwrap_or(false),
        category_smartphone: row["category_smartphone"].as_bool().unwrap_or(false),
        country_jp: row["country_jp"].as_str().map(String::from),
        country_en: row["country_en"].as_str().map(String::from),
        prefecture_jp: row["prefecture_jp"].as_str().map(String::from),
        prefecture_en: row["prefecture_en"].as_str().map(String::from),
        post_no_jp: row["post_no_jp"].as_str().map(String::from),
        post_no_en: row["post_no_en"].as_str().map(String::from),
        address_1_jp: row["address_1_jp"].as_str().map(String::from),
        address_1_en: row["address_1_en"].as_str().map(String::from),
        address_2_jp: row["address_2_jp"].as_str().map(String::from),
        address_2_en: row["address_2_en"].as_str().map(String::from),
        address_other_jp: row["address_other_jp"].as_str().map(String::from),
        address_other_en: row["address_other_en"].as_str().map(String::from),
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
        //created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
        //updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
    }
}


pub async fn handle_list_todos(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let query = "SELECT * FROM todo16 ORDER BY id DESC";
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
    
    let query = "INSERT INTO todo16 
    (title, content, content_type, public_type, food_orange, food_apple, food_banana, food_melon, food_grape, category_food, category_drink, category_gadget, category_sport, category_government, category_internet, category_smartphone, country_jp, country_en, prefecture_jp, prefecture_en, post_no_jp, post_no_en, address_1_jp, address_1_en, address_2_jp, address_2_en, address_other_jp, address_other_en, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6) 
    VALUES 
    (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
    RETURNING *";
    let stmt = db.prepare(query)
        .bind(&[
            create_req.title.into(),
            create_req.content.into(),
            create_req.content_type.into(),
            create_req.public_type.into(),
            create_req.food_orange.into(),
            create_req.food_apple.into(),
            create_req.food_banana.into(),
            create_req.food_melon.into(),
            create_req.food_grape.into(),
            create_req.category_food.into(),
            create_req.category_drink.into(),
            create_req.category_gadget.into(),
            create_req.category_sport.into(),
            create_req.category_government.into(),
            create_req.category_internet.into(),
            create_req.category_smartphone.into(),
            create_req.country_jp.into(),
            create_req.country_en.into(),
            create_req.prefecture_jp.into(),
            create_req.prefecture_en.into(),
            create_req.post_no_jp.into(),
            create_req.post_no_en.into(),
            create_req.address_1_jp.into(),
            create_req.address_1_en.into(),
            create_req.address_2_jp.into(),
            create_req.address_2_en.into(),
            create_req.address_other_jp.into(),
            create_req.address_other_en.into(),
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
            //now.clone().into(),
            //now.clone().into(),
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
    
    let query = "DELETE FROM todo16 WHERE id = ?";
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
    
    let check_query = "SELECT id FROM todo16 WHERE id = ?";
    let check_stmt = db.prepare(check_query).bind(&[update_req.id.into()])?;
    let exists = check_stmt.first::<serde_json::Value>(None).await?;
    
    if exists.is_none() {
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
    if let Some(public_type) = update_req.public_type {
        query_parts.push("public_type = ?");
        bindings.push(public_type.into());
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
    if let Some(category_food) = update_req.category_food {
        query_parts.push("category_food = ?");
        bindings.push(category_food.into());
    }
    if let Some(category_drink) = update_req.category_drink {
        query_parts.push("category_drink = ?");
        bindings.push(category_drink.into());
    }
    if let Some(category_gadget) = update_req.category_gadget {
        query_parts.push("category_gadget = ?");
        bindings.push(category_gadget.into());
    }
    if let Some(category_sport) = update_req.category_sport {
        query_parts.push("category_sport = ?");
        bindings.push(category_sport.into());
    }
    if let Some(category_government) = update_req.category_government {
        query_parts.push("category_government = ?");
        bindings.push(category_government.into());
    }
    if let Some(category_internet) = update_req.category_internet {
        query_parts.push("category_internet = ?");
        bindings.push(category_internet.into());
    }
    if let Some(category_smartphone) = update_req.category_smartphone {
        query_parts.push("category_smartphone = ?");
        bindings.push(category_smartphone.into());
    }
    if let Some(country_jp) = update_req.country_jp {
        query_parts.push("country_jp = ?");
        bindings.push(country_jp.into());
    }
    if let Some(country_en) = update_req.country_en {
        query_parts.push("country_en = ?");
        bindings.push(country_en.into());
    }
    if let Some(prefecture_jp) = update_req.prefecture_jp {
        query_parts.push("prefecture_jp = ?");
        bindings.push(prefecture_jp.into());
    }
    if let Some(prefecture_en) = update_req.prefecture_en {
        query_parts.push("prefecture_en = ?");
        bindings.push(prefecture_en.into());
    }
    if let Some(post_no_jp) = update_req.post_no_jp {
        query_parts.push("post_no_jp = ?");
        bindings.push(post_no_jp.into());
    }
    if let Some(post_no_en) = update_req.post_no_en {
        query_parts.push("post_no_en = ?");
        bindings.push(post_no_en.into());
    }
    if let Some(address_1_jp) = update_req.address_1_jp {
        query_parts.push("address_1_jp = ?");
        bindings.push(address_1_jp.into());
    }
    if let Some(address_1_en) = update_req.address_1_en {
        query_parts.push("address_1_en = ?");
        bindings.push(address_1_en.into());
    }
    if let Some(address_2_jp) = update_req.address_2_jp {
        query_parts.push("address_2_jp = ?");
        bindings.push(address_2_jp.into());
    }
    if let Some(address_2_en) = update_req.address_2_en {
        query_parts.push("address_2_en = ?");
        bindings.push(address_2_en.into());
    }
    if let Some(address_other_jp) = update_req.address_other_jp {
        query_parts.push("address_other_jp = ?");
        bindings.push(address_other_jp.into());
    }
    if let Some(address_other_en) = update_req.address_other_en {
        query_parts.push("address_other_en = ?");
        bindings.push(address_other_en.into());
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
    
    //query_parts.push("updated_at = ?");
    //bindings.push(now.into());
    bindings.push(update_req.id.into());
    
    let query = format!("UPDATE todo16 SET {} WHERE id = ? RETURNING *", query_parts.join(", "));
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
