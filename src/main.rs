use actix_web::{main, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CustomField {
    HashMapString(std::collections::HashMap<String, Value>),
    String(String),
}

#[derive(Serialize, Deserialize)]
struct Line {
    quantity: i64,
}

#[derive(Serialize, Deserialize)]
struct Activity {
    id: i64,
    name: String,
    custom_field: CustomField,
    custom_field2: CustomField,
    created_at: String,
    lines: Vec<Line>,
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[post("/realtime")]
async fn echo(req_body: web::Json<Activity>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "activity processed: {} with custom field: {:?}",
        req_body.id, req_body.custom_field
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use actix_web::{body::to_bytes, dev::Service, http, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_realtime() {
        let app = test::init_service(App::new().service(echo)).await;

        let mut json_custom_field: HashMap<String, Value> = HashMap::new();
        json_custom_field.insert("test".to_string(), Value::Bool(true));

        let req = test::TestRequest::post()
            .uri("/realtime")
            .set_json(&Activity {
                id: 123,
                name: "test".to_string(),
                custom_field: CustomField::HashMapString(json_custom_field),
                custom_field2: CustomField::String("test".to_string()),
                created_at: "2022-07-12T10:00:00Z".to_string(),
                lines: vec![Line { quantity: 1 }, Line { quantity: 3 }],
            })
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        let expected_body =
            b"activity processed: 123 with custom field: HashMapString({\"test\": Bool(true)})";

        assert_eq!(body_bytes, web::Bytes::from_static(expected_body));
    }
}
