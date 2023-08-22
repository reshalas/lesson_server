use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{LocationData, Subjects};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub class: i16,
    pub school: i32,
    pub email: String,
    pub location_data: LocationData,
}

#[derive(Deserialize)]
pub struct TaskDTO {
    pub task: String,
    pub price: f64,
    pub subject: Subjects,
    pub target_finishing_time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct SlotDTO {
    pub subject: Subjects,
    pub limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct SingDto {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SlotLimitDTO {
    pub limit: Option<i32>,
}
