use actix_web::dev::HttpResponseBuilder;
use actix_web::http;
use actix_web::HttpResponse;
pub fn unauthorized_request() -> HttpResponse {
    return HttpResponseBuilder::new(http::StatusCode::from_u16(401).unwrap())
        .header(http::header::CONTENT_TYPE, "application/json")
        .body("Unauthorized Request");
}
