
use actix_web::HttpResponse;
use json;
use json::JsonValue;

pub fn error(e:String) -> HttpResponse {
    let result = json::object! {
        "result" => "error",
        "err" => e
    };
    return HttpResponse::Ok()
    .content_type("application/json")
    .body(result.dump());
}

pub fn success() -> HttpResponse {
    let result = json::object! {
        "result" => "success"
    };
    return HttpResponse::Ok()
    .content_type("application/json")
    .body(result.dump());
}

pub fn success_with_data(data: JsonValue) -> HttpResponse {
    let result = json::object! {
        "result" => "success",
        "data" => data
    };
    return HttpResponse::Ok()
    .content_type("application/json")
    .body(result.dump());
}
