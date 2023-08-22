use crate::{
    repositoryes::*,
    types::{
        dto::*,
        errors::{RegistrationError, SingUpError},
        *,
    },
    utils::{build_register_mesage, send_email},
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::types::Uuid;
use std::str::FromStr;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&UserSRepository::all().await).unwrap())
}

fn extract_value_from_headers(req: &HttpRequest, name: &str) -> Result<String, ()> {
    match req.headers().get(name) {
        Some(value) => Ok(value.to_str().unwrap().to_string()),
        None => Err(()),
    }
}

pub fn extract_sing_data(req: &HttpRequest) -> Result<SingDto, HttpResponse> {
    let username = extract_value_from_headers(req, "username");
    let password = extract_value_from_headers(req, "password");
    match (username.clone(), password) {
        (Ok(username), Ok(password)) => Ok(SingDto { username, password }),
        (Err(_), Err(_)) => Err(HttpResponse::BadRequest()
            .body(serde_json::to_string(&SingUpError::NoHeaders).unwrap())),
        _ => {
            if let Err(_) = username {
                return Err(HttpResponse::BadRequest()
                    .body(serde_json::to_string(&SingUpError::NoUsernameHeader).unwrap()));
            }
            Err(HttpResponse::BadRequest()
                .body(serde_json::to_string(&SingUpError::NoPasswordHeader).unwrap()))
        }
    }
}

#[get("/get")]
pub async fn get(req: HttpRequest) -> impl Responder {
    let sing_dto = extract_sing_data(&req);
    if let Err(resp) = sing_dto {
        return resp;
    }
    let sing_dto = sing_dto.unwrap();
    let username = sing_dto.username.clone();
    let user = User::from_dto(sing_dto).await;
    match user {
        Some(user) => HttpResponse::Ok().body(serde_json::to_string(&user).unwrap()),
        None => {
            if UserSRepository::is_username_free(username.clone()).await {
                return HttpResponse::NotFound()
                    .body(serde_json::to_string(&SingUpError::NoUser).unwrap());
            }
            if User::from_username(username, Some(false)).await.is_some() {
                return HttpResponse::NotFound()
                    .body(serde_json::to_string(&SingUpError::UserIsNotActivated).unwrap());
            }
            HttpResponse::NotFound()
                .body(serde_json::to_string(&SingUpError::WrongPassword).unwrap())
        }
    }
}

#[get("/is_free/username/{username}")]
pub async fn check_username(username: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        serde_json::to_string(&UserSRepository::is_username_free((*username).clone()).await)
            .unwrap(),
    )
}

#[get("/is_free/email/{email}")]
pub async fn check_email(email: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        serde_json::to_string(&UserSRepository::is_email_free((*email).clone()).await).unwrap(),
    )
}

#[post("/register")]
pub async fn register(dto: web::Json<UserDTO>) -> impl Responder {
    let mut errors: Vec<RegistrationError> = Vec::new();

    if dto.username.len() > 128 || dto.username.len() < 2 {
        errors.push(RegistrationError::InvalidUsername)
    }
    if dto.password.len() > 50 || dto.password.len() < 8 {
        errors.push(RegistrationError::InvalidPassword)
    }
    if dto.class > 11 || dto.class < 1 {
        errors.push(RegistrationError::InvalidClass)
    }
    if dto.school < 1 {
        errors.push(RegistrationError::InvalidSchoolNumber)
    }
    if dto.location_data.city.is_empty() || dto.location_data.country.is_empty() {
        errors.push(RegistrationError::InvalidLocation)
    }
    if !UserSRepository::is_username_free(dto.username.clone()).await {
        errors.push(RegistrationError::UsernameExistsInDb)
    }
    if !UserSRepository::is_email_free(dto.email.clone()).await {
        errors.push(RegistrationError::EmailExistsInDB)
    }
    if !errors.is_empty() {
        return HttpResponse::BadRequest().body(serde_json::to_string(&errors).unwrap());
    }
    let (user, verefication_key) = User::register(dto.0).await.unwrap();
    send_email(
        user.email().clone(),
        build_register_mesage(
            user.last_name().clone(),
            &verefication_key,
            &user.username(),
        ),
    )
    .await
    .unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&user).unwrap())
}

#[post("/rate/{mark}/user/{user}")]
pub async fn rate(dto: web::Path<(i16, String)>) -> impl Responder {
    let (mark, user) = dto.clone();
    if mark > 5 || mark < 1 {
        return HttpResponse::BadRequest().finish();
    }
    match User::from_username(user, Some(true)).await {
        Some(mut user) => {
            user.rate(mark).await;
            HttpResponse::Ok().body("ok")
        }
        None => HttpResponse::NotFound().body("user doesn't exist"),
    }
}

#[get("/is_free/email/{email}")]
pub async fn is_email_free(email: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        UserSRepository::is_email_free(email.clone())
            .await
            .to_string(),
    )
}

#[get("/is_free/username/{username}")]
pub async fn is_username_free(username: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        UserSRepository::is_username_free(username.clone())
            .await
            .to_string(),
    )
}

#[post("/change/email/{email}")]
pub async fn change_email(req: HttpRequest, email: web::Path<String>) -> impl Responder {
    let sing_dto = extract_sing_data(&req);
    if let Err(resp) = sing_dto {
        return resp;
    }
    let sing_dto = sing_dto.unwrap();
    let user = User::from_dto(sing_dto).await;
    match user {
        Some(mut user) => {
            user.change_email(email.clone()).await;
            return HttpResponse::Ok().body("ok");
        }
        None => return HttpResponse::NotFound().finish(),
    }
}

#[post("/accept_task/{task}")]
pub async fn accept_task(uuid: web::Path<String>, req: HttpRequest) -> impl Responder {
    let task = match get_task_for_accepting(uuid).await {
        Ok(task) => task,
        Err(err) => return err,
    };

    //Получаем пользователя
    let sing_dto = extract_sing_data(&req);
    if let Err(resp) = sing_dto {
        return resp;
    }
    let sing_dto = sing_dto.unwrap();
    let user = User::from_dto(sing_dto).await;
    if let None = user {
        return HttpResponse::BadRequest().body("User doesn't exists");
    }
    let mut user = user.unwrap();

    match user.slots_component().accept_task(task).await {
        Ok(transaction) => HttpResponse::Ok().body(serde_json::to_string(&transaction).unwrap()),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

async fn get_task_for_accepting(uuid: web::Path<String>) -> Result<Task, HttpResponse> {
    let task_uuid = Uuid::from_str(&uuid);
    if let Err(_) = task_uuid {
        return Err(HttpResponse::BadRequest().body("Bad user uuid"));
    }
    let task_uuid = task_uuid.unwrap();
    let task = TaskSRepository::get(task_uuid).await;
    match task {
        Some(task) => Ok(task),
        None => Err(HttpResponse::BadRequest().body("Task doesn't exists")),
    }
}
