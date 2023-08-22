use crate::{
    repositoryes::*,
    types::{
        dto::{SingDto, TaskDTO},
        User,
    },
};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&TaskSRepository::all().await).unwrap())
}

#[get("/tasks")]
pub async fn get_all_tasks(user: web::Query<SingDto>) -> impl Responder {
    let user = User::from_dto(user.0).await;
    if let Some(user) = user {
        return HttpResponse::Ok()
            .body(serde_json::to_string(&TaskSRepository::from_owner(&user).await).unwrap());
    }
    HttpResponse::BadRequest().body("Owner doesn't exists")
}

#[post("/publish_task")]
pub async fn publish_task(user: web::Query<SingDto>, dto: web::Json<TaskDTO>) -> impl Responder {
    let user = User::from_dto(user.0).await;
    if let Some(mut user) = user {
        return HttpResponse::Ok().body(
            serde_json::to_string(&user.tasks_component().publish_task(dto.0).await).unwrap(),
        );
    }
    HttpResponse::BadRequest().body("Owner doesn't exists")
}
