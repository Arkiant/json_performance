mod process;

use actix_web::{main, post, web, App, HttpResponse, HttpServer, Responder};

use crate::process::Activity;

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
    use crate::process::{Activity, CustomField, Line};

    #[actix_web::test]
    async fn test_realtime() {
        let app = test::init_service(App::new().service(echo)).await;

        let mut json_custom_field: HashMap<String, serde_json::Value> = HashMap::new();
        json_custom_field.insert("test".to_string(), serde_json::Value::Bool(true));

        let activity: Activity = Activity::new(
            123,
            "
            test"
                .to_string(),
            CustomField::HashMapString(json_custom_field),
            CustomField::String("test".to_string()),
            "2022-07-12T10:00:00Z".to_string(),
            vec![Line::new(1), Line::new(1)],
        );

        let req = test::TestRequest::post()
            .uri("/realtime")
            .set_json(&activity)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();

        let expected_body =
            b"activity processed: 123 with custom field: HashMapString({\"test\": Bool(true)})";

        assert_eq!(body_bytes, web::Bytes::from_static(expected_body));
    }
}
