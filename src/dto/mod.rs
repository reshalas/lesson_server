use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::types::{LocationData, Subjects, TelegramData};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub class: i16,
    pub school: i32,
    pub email: Option<String>,
    pub phone_number: Option<String>,

    pub telegram_data: TelegramData,
    pub location_data: LocationData,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDTO {
    pub task: String,
    pub price: f64,
    pub subject: Subjects,
    pub target_finishing_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct SlotDTO {
    pub subject: Subjects,
    pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct EmailDTO {
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PhoneDTO {
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SlotLimitDTO{
    pub limit: Option<i32>
}
