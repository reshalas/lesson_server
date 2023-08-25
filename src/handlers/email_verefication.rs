use actix_web::{get, web::Path, HttpResponse, Responder};

use crate::{
    repositoryes::UserSRepository, types::errors::VerfyEmailError, utils::get_company_email,
};

enum Action {
    Verfy,
    Cancel,
}

#[get("/verfy/{username}/{verfy_key}")]
pub async fn verfy(args: Path<(String, String)>) -> impl Responder {
    let (username, vkey) = args.clone();
    match UserSRepository::verfy_email(&vkey, &username).await {
        Ok(_) => HttpResponse::Ok().body(build_succes_page(Action::Verfy)),
        Err(err) => HttpResponse::BadRequest().body(build_error_page(err, Action::Verfy)),
    }
}

fn build_error_page(error: VerfyEmailError, action: Action) -> String {
    match error {
        VerfyEmailError::EmailAlreadyVerfied=>{match action {
            Action::Verfy=>format!(
            "
            <div>
                <h1>Уже усе активировано, не парься!</h1>
            </div>"),
            Action::Cancel=>format!("
            <div>
                <h1>Кто-то уже подтвердил ваш Email!</h1>
                <p> Если вы просто решили нажать вторую кнопку, то ничего страшного, мы никому не расскажем, что вы великий
                    экспериментатор. В противном случае напиши нам на почту и расскажи чутка об этом случае.
                </p>
            </div>")
        }},
        VerfyEmailError::WrongVerfyKey=>format!("
            <div>
                <h1>Обращение к юнным хакерам!</h1>
                <p> Ты огромный молодец, что пытаешься изучить работу нашего сайта. 
                    Давай, так если ты сможешь найти баг, и хорошо описать путь его повторения, пиши на нашу почту: {}.
                    Если баг-репорт будет полным, то я переведу тебе 10к. Пойдет? Так что вот тебе ещё мотивация. 
                    Если от тебя будет очень много помощи, то у тебя будет нихилый такой шанс попасть в нашу команду разработчиков.
                </p>
            </div>", get_company_email())
    }
}

fn build_succes_page(action: Action) -> String {
    match action {
        Action::Verfy => format!(
            "<div>
                <h1>Добро пожаловать!</h1>
                <p> Твой аккаунт активирован, теперь ты можешь войти в свой аккаунт на сайте</p>
                <a href=\"{}\"></a>
            </div>",
            path_to_frontend()
        ),
        Action::Cancel => format!("<div>
            <h1>Запрос на регистрацию успешно заблокирован!</h1>
            <p> Но все же если ты учишься в школе, обрати внимание <a href=\"{}\">на нащ сайт</a></p>
        </div>", path_to_frontend()),
    }
}

fn path_to_frontend() -> String {
    dotenvy::var("PATH_TO_FRONTEND").unwrap()
}

#[get("/cancel/{username}/{verfy_key}")]
pub async fn cancel(args: Path<(String, String)>) -> impl Responder {
    let (username, vkey) = args.clone();
    match UserSRepository::cancel_verefication(&vkey, &username).await {
        Ok(_) => HttpResponse::Ok().body(build_succes_page(Action::Cancel)),
        Err(err) => HttpResponse::BadRequest().body(build_error_page(err, Action::Cancel)),
    }
}
