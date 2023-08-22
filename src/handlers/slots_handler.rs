use crate::{
    handlers::user_s_handler,
    repositoryes::*,
    types::{dto::*, Subjects, User},
};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::types::Uuid;
use std::str::FromStr;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&SlotSRepository::all().await).unwrap())
}

#[get("/user")]
pub async fn get_all_by_owner(req: HttpRequest) -> impl Responder {
    let sing_data = user_s_handler::extract_sing_data(&req);
    if let Err(resp) = sing_data {
        return resp;
    }
    let sing_data = sing_data.unwrap();
    let user = User::from_dto(sing_data).await;
    if let Some(user) = user {
        return HttpResponse::Ok()
            .body(serde_json::to_string(&SlotSRepository::from_owner(&user).await).unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[get("/slots")]
pub async fn get_slots(req: HttpRequest) -> impl Responder {
    let sing_data = user_s_handler::extract_sing_data(&req);
    if let Err(resp) = sing_data {
        return resp;
    }
    let sing_data = sing_data.unwrap();
    let user = UserSRepository::get(&sing_data).await;
    if let Some(mut user) = user {
        return HttpResponse::Ok()
            .body(serde_json::to_string(&user.slots_component().slots()).unwrap());
    }
    HttpResponse::BadRequest().body("Owner doesn't exists")
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<String>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        let slot = SlotSRepository::get(uuid).await;
        match slot {
            Some(slot) => return HttpResponse::Ok().body(serde_json::to_string(&slot).unwrap()),
            None => return HttpResponse::NotFound().finish(),
        }
    }
    HttpResponse::NotFound().body("Bad uuid")
}

#[post("/{id}/update_limit")]
pub async fn update_limit(id: web::Path<String>, limit: web::Json<SlotLimitDTO>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(id.as_str()) {
        let slot = SlotSRepository::get(uuid).await;
        match slot {
            Some(mut slot) => {
                slot.update_limit(limit.limit).await;
                return HttpResponse::Ok().finish();
            }
            None => return HttpResponse::NotFound().finish(),
        }
    }
    HttpResponse::NotFound().body("Bad uuid")
}

#[get("/{id}/owner")]
pub async fn get_owner(id: web::Path<String>) -> impl Responder {
    let uuid = Uuid::from_str(id.as_str());
    if let Ok(uuid) = uuid {
        let slot = SlotSRepository::get(uuid).await;
        if let Some(slot) = slot {
            return HttpResponse::Ok().body(serde_json::to_string(&slot.owner().await).unwrap());
        }
        return HttpResponse::BadRequest().body("Owner doesn't exists");
    }

    HttpResponse::BadRequest().body("Bad owner id")
}

#[post("/slots/activate")]
pub async fn activate_slot(req: HttpRequest, dto: web::Json<SlotDTO>) -> impl Responder {
    let sing_data = user_s_handler::extract_sing_data(&req);
    if let Err(resp) = sing_data {
        return resp;
    }
    let sing_data = sing_data.unwrap();
    let user = User::from_dto(sing_data).await;
    if let Some(mut user) = user {
        if let Err(_) = user.slots_component().add_slot(dto.0).await {
            return HttpResponse::BadRequest().body("Slot has already activated");
        }
        return HttpResponse::Ok().body(serde_json::to_string(&user).unwrap());
    }
    HttpResponse::BadRequest().body("Owner doesn't exists")
}

#[delete("/slots/deactivate/{subject}")]
pub async fn deactivate_slot(
    req: HttpRequest,
    info: web::Path<(String, Subjects)>,
) -> impl Responder {
    let sing_data = user_s_handler::extract_sing_data(&req);
    if let Err(resp) = sing_data {
        return resp;
    }
    let sing_data = sing_data.unwrap();
    let user = User::from_dto(sing_data).await;
    if let Some(mut user) = user {
        if let Err(_) = user.slots_component().remove_slot(info.1).await {
            return HttpResponse::BadRequest().body("Slot isn't activated or can't be deactivated");
        }
        return HttpResponse::Ok().body(serde_json::to_string(&user).unwrap());
    }
    HttpResponse::BadRequest().body("Owner doesn't exists")
}
