use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::types::Uuid;
use std::str::FromStr;

use crate::{
    repositoryes::{SlotSRepository, TransactionSRepository},
    types::User,
};

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&TransactionSRepository::all().await).unwrap())
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<String>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        let transaction = TransactionSRepository::get(uuid).await;
        match transaction {
            Some(user) => return HttpResponse::Ok().body(serde_json::to_string(&user).unwrap()),
            None => return HttpResponse::NotFound().finish(),
        }
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[get("/sender/{slot_id}")]
pub async fn get_all_by_sender_id(id: web::Path<String>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        if let Some(slot) = SlotSRepository::get(uuid).await {
            return HttpResponse::Ok().body(
                serde_json::to_string(&TransactionSRepository::all_by_sender_slot(&slot).await)
                    .unwrap(),
            );
        }
        return HttpResponse::BadRequest().body("Slot doesnt exist");
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[get("/sender/finished/{slot_id}")]
pub async fn get_all_finished_by_sender_id(id: web::Path<String>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        if let Some(slot) = SlotSRepository::get(uuid).await {
            return HttpResponse::Ok().body(
                serde_json::to_string(
                    &TransactionSRepository::finished_by_sender_slot(&slot).await,
                )
                .unwrap(),
            );
        }
        return HttpResponse::BadRequest().body("Slot doesnt exist");
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[get("/sender/unfinished/{slot_id}")]
pub async fn get_all_unfinished_by_sender_id(id: web::Path<String>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        if let Some(slot) = SlotSRepository::get(uuid).await {
            return HttpResponse::Ok().body(
                serde_json::to_string(
                    &TransactionSRepository::unfinished_by_sender_slot(&slot).await,
                )
                .unwrap(),
            );
        }
        return HttpResponse::BadRequest().body("Slot doesnt exist");
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[get("/recipient/{recipient_id}")]
pub async fn get_all_by_recipient_id(recipient: web::Path<String>) -> impl Responder {
    if let Some(user) = User::from_username(recipient.clone(), Some(true)).await {
        return HttpResponse::Ok().body(
            serde_json::to_string(&TransactionSRepository::all_by_recipient(&user).await).unwrap(),
        );
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[get("/recipient/finished/{recipient}")]
pub async fn get_all_finished_by_recipient_id(recipient: web::Path<String>) -> impl Responder {
    if let Some(user) = User::from_username(recipient.clone(), None).await {
        return HttpResponse::Ok().body(
            serde_json::to_string(&TransactionSRepository::finished_by_recipient_id(&user).await)
                .unwrap(),
        );
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[get("/recipient/unfinished/{recipient_id}")]
pub async fn get_all_unfinished_by_recipient_id(username: web::Path<String>) -> impl Responder {
    if let Some(user) = User::from_username(username.clone(), None).await {
        return HttpResponse::Ok().body(
            serde_json::to_string(&TransactionSRepository::unfinished_by_reciepment(&user).await)
                .unwrap(),
        );
    }
    HttpResponse::BadRequest().body("Bad uuid")
}

#[post("/{id}/finish")]
pub async fn finish(id: web::Path<String>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        if let Some(transaction) = TransactionSRepository::get(uuid).await {
            transaction.finish().await;
            return HttpResponse::Created().finish();
        }
        return HttpResponse::NotFound().finish();
    }
    HttpResponse::BadRequest().body("Bad uuid")
}
