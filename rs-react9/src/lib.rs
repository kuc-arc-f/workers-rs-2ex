use serde::{Deserialize, Serialize};
use worker::*;
use reqwest::Error;
use serde_json::json;
use serde_json::Value;

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
    data: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoCreateResponse {
    status: u16,
    data: Todo,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let api_host = env.var("API_HOST")?.to_string();
    println!("API_HOST = {}", api_host);

    Router::new()

        .get_async("/api/list", handle_list_todos)
        .post_async("/api/create", handle_create_todo)
        .post_async("/api/delete", handle_delete_todo)
        .post_async("/api/update", handle_update_todo)

        .get_async("/foo", handle_get)
        .get_async("/test", handle_test)
        .run(req, env)
        .await
}

pub async fn handle_test(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let api_host = _ctx.env.var("API_HOST")?.to_string();
    //println!("API_HOST = {}", api_host);
    console_log!("API_HOST = {}", api_host);

    let mut response = Response::from_html("OK")?;
    // 必要に応じてヘッダーをカスタム可能
    response.headers_mut()
        .set("Content-Type", "text/html; charset=utf-8")?;
    Ok(response)
}

/**
*
* @param
*
* @return
*/
pub async fn get_external_api(url: String) -> String {
  println!("url={}", url );

  let client = reqwest::Client::new();

  // GETリクエストを送信
  let response = client.get(url).send().await;
  //println!("response: {:?}", response);
  // HTTPステータスコードを取得
  match response {
    Ok(resp) => {
      let status = resp.status();
      println!("HTTP Status: {}", status.to_string());
      if status.is_success() {
        println!("Request was successful!");
        let body = resp.text().await.unwrap();
        println!("Response.body: {}", body);
        let payload = json!({
          "status": 200,
          "body": &body.to_string()
        });
        //println!("payload: {}", payload);
        return payload.to_string();
      } else if status.is_client_error() {
        println!("Client error occurred!");
        let body = resp.text().await.unwrap();
        let payload = json!({
          "status": 400,
          "body": &body.to_string()
        });
        return payload.to_string();
      } else if status.is_server_error() {
        println!("Server error occurred!");
        let body = resp.text().await.unwrap();
        let payload = json!({
          "status": 500,
          "body": &body.to_string()
        });
        return payload.to_string();
      }
    }
    Err(err) => {
        eprintln!("Request failed: {}", err);
    }
  }
  return "".to_string();
}

/**
*
* @param
*
* @return
*/
pub async fn post_external_api(url: String, data: String) -> String {
  let client = reqwest::Client::new();

  // JSON文字列をValue型にデコード
  let value: Value = serde_json::from_str(&data).expect("REASON");
  println!("url={}", url );

  // POSTリクエストを送信
  let response = client.post(url)
      .json(&value)
      .send()
      .await;

  match response {
    Ok(resp) => {
        let status = resp.status();
        println!("HTTP Status: {}", status.to_string());

        if status.is_success() {
            println!("Request was successful!");
            let body = resp.text().await.unwrap();
            let payload = json!({
              "status": 200,
              "body": &body.to_string()
            });
            //println!("payload: {}", payload);
            return payload.to_string();
        }
        else if status.is_client_error() {
            println!("Client error occurred!");
            let body = resp.text().await.unwrap();
            let payload = json!({
              "status": 400,
              "body": &body.to_string()
            });
            return payload.to_string();
        } else if status.is_server_error() {
            println!("Server error occurred!");
            let body = resp.text().await.unwrap();
            let payload = json!({
              "status": 500,
              "body": &body.to_string()
            });
            return payload.to_string();
        }
    }
    Err(err) => {
        eprintln!("Request failed: {}", err);
    }
  }      
  return "".to_string();
}

/**
*
* @param
*
* @return
*/
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

#[derive(Debug, Deserialize, Serialize)]
struct ListTodoResponse {
    status: u32,
    body: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct BaseTodoResponse {
    status: u32,
    body: String,
}

/**
*
* @param
*
* @return
*/
pub async fn handle_list_todos(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let api_host = ctx.env.var("API_HOST")?.to_string();
    console_log!("API_HOST = {}", api_host);
    let url = api_host + "/api/todo11";

    let resp = get_external_api(url).await;
    //console_log!("resp= {}", resp);
    match serde_json::from_str::<ListTodoResponse>(&resp) {
        Ok(data) => {
            // デコード成功！
            //console_log!("body: {}", data.body);
            console_log!("status: {}", data.status);
            if data.status != 200 {
                return Response::from_json(&TodoListResponse {
                    status: 500,
                    data: "".to_string(),
                })
            }else {
                return Response::from_json(&TodoListResponse {
                    status: 200,
                    data: data.body,
                })
            }
        }
        Err(e) => {
            eprintln!("JSONデコードエラー: {}", e);
        }
    }

    Response::from_json(&TodoListResponse {
        status: 500,
        data: "".to_string(),
    })
}
/**
*
* @param
*
* @return
*/
pub async fn handle_create_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let create_req: CreateTodoRequest = req.json().await?;

    let api_host = ctx.env.var("API_HOST")?.to_string();
    console_log!("API_HOST = {}", api_host);
    let url = api_host + "/api/todo11";

    //console_log!("{:?}",create_req);
    let dataJson = serde_json::to_string(&create_req).unwrap();
    let resp = post_external_api(url, dataJson).await;
    match serde_json::from_str::<ListTodoResponse>(&resp) {
        Ok(data) => {
            // デコード成功
            console_log!("status: {}", data.status);
            if data.status != 200 {
                return Response::from_json(&TodoListResponse {
                    status: 500,
                    data: "".to_string(),
                })
            }else {
                return Response::from_json(&TodoListResponse {
                    status: 200,
                    data: data.body,
                })
            }
        }
        Err(e) => {
            eprintln!("JSONデコードエラー: {}", e);
        }
    }

    Response::from_json(&TodoListResponse {
        status: 500,
        data: "".to_string(),
    })
}
/**
*
* @param
*
* @return
*/
pub async fn handle_delete_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let delete_req: DeleteTodoRequest = req.json().await?;

    let api_host = ctx.env.var("API_HOST")?.to_string();
    console_log!("API_HOST = {}", api_host);
    let url = api_host + "/api/todo11/delete";

    let dataJson = serde_json::to_string(&delete_req).unwrap();
    let resp = post_external_api(url, dataJson).await;

    Response::from_json(&GenericResponse {
        status: 200,
        message: "Todo deleted successfully".to_string(),
    })
}
/**
*
* @param
*
* @return
*/
pub async fn handle_update_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let update_req: UpdateTodoRequest = req.json().await?;

    let api_host = ctx.env.var("API_HOST")?.to_string();
    console_log!("API_HOST = {}", api_host);
    let url = api_host + "/api/todo11/update";

    let dataJson = serde_json::to_string(&update_req).unwrap();
    let resp = post_external_api(url, dataJson).await;

    Response::from_json(&GenericResponse {
        status: 200,
        message: "Todo deleted successfully".to_string(),
    })
}