use actix_web::dev::HttpResponseBuilder;
use actix_web::http;
use actix_web::{web::Json, HttpResponse};
use serde::{Deserialize, Serialize};
extern crate serde_json;

pub fn unauthorized_request() -> HttpResponse {
        return HttpResponseBuilder::new(http::StatusCode::from_u16(401).unwrap())
                .header(http::header::CONTENT_TYPE, "application/json")
                .body("Unauthorized Request");
}
#[derive(Serialize, Deserialize)]
struct DataTemplate<T> {
        status: i32,
        data: T,
}
#[derive(Serialize, Deserialize)]
struct ErrorTemplate {
        status: i32,
        error: String,
}
//Try using serde_json
pub fn data(dt: serde_json::Value) -> HttpResponse {
        // let json_dt=serde_json::
        let data = DataTemplate::<serde_json::Value> {
                status: 200,
                data: dt,
        };
        return HttpResponseBuilder::new(http::StatusCode::from_u16(200).unwrap())
                .header(http::header::CONTENT_TYPE, "application/json")
                .json(data);
}
pub fn error(errorcode: i32, message: String) -> HttpResponse {
        let data = ErrorTemplate {
                status: errorcode,
                error: message,
        };
        return HttpResponseBuilder::new(http::StatusCode::from_u16(400).unwrap())
                .header(http::header::CONTENT_TYPE, "application/json")
                .json(data);
}
