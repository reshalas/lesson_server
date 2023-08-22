use actix_web::Scope;

mod email_verefication;
mod slots_handler;
mod tasks_handler;
mod transactions_handler;
mod user_s_handler;

pub fn build_user_scope() -> Scope {
    Scope::new("/users")
        .service(user_s_handler::get_all)
        .service(user_s_handler::get)
        .service(user_s_handler::register)
        .service(user_s_handler::rate)
        .service(user_s_handler::change_email)
        .service(user_s_handler::check_username)
        .service(user_s_handler::check_email)
        .service(tasks_handler::publish_task)
        .service(tasks_handler::get_all_tasks)
        .service(user_s_handler::accept_task)
        .service(slots_handler::get_slots)
        .service(slots_handler::activate_slot)
        .service(slots_handler::deactivate_slot)
        .service(user_s_handler::is_email_free)
        .service(user_s_handler::is_username_free)
}

pub fn build_email_verefication_scope() -> Scope {
    Scope::new("/verefication/email")
        .service(email_verefication::verfy)
        .service(email_verefication::cancel)
}

pub fn build_tasks_scope() -> Scope {
    Scope::new("/tasks").service(tasks_handler::get_all)
}

pub fn build_slots_scope() -> Scope {
    Scope::new("/slots")
        .service(slots_handler::get_all)
        .service(slots_handler::get_by_id)
        .service(slots_handler::get_owner)
        .service(slots_handler::update_limit)
        .service(slots_handler::get_all_by_owner)
}

pub fn build_transactions_scope() -> Scope {
    Scope::new("/transactions")
        .service(transactions_handler::get_all)
        .service(transactions_handler::get_by_id)
        .service(transactions_handler::get_all_by_recipient_id)
        .service(transactions_handler::get_all_by_sender_id)
        .service(transactions_handler::get_all_finished_by_recipient_id)
        .service(transactions_handler::get_all_finished_by_sender_id)
        .service(transactions_handler::get_all_unfinished_by_sender_id)
        .service(transactions_handler::get_all_finished_by_recipient_id)
}
