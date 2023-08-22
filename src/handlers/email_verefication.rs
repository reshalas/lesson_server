use actix_web::{post, web::Path, HttpResponse, Responder};

use crate::repositoryes::UserSRepository;

#[post("/verfy/{username}/{verfy_key}")]
pub async fn verfy(args: Path<(String, String)>) -> impl Responder {
    let (username, vkey) = args.clone();
    match UserSRepository::verfy_email(&vkey, &username).await {
        Ok(_) => HttpResponse::Ok().body(String::new()),
        Err(err) => HttpResponse::BadRequest().body(serde_json::to_string(&err).unwrap()),
    }
}

#[post("/cancel/{username}/{verfy_key}")]
pub async fn cancel(args: Path<(String, String)>) -> impl Responder {
    let (username, vkey) = args.clone();
    match UserSRepository::cancel_verefication(&vkey, &username).await {
        Ok(_) => HttpResponse::Ok().body(String::new()),
        Err(err) => HttpResponse::BadRequest().body(serde_json::to_string(&err).unwrap()),
    }
}
