use std::str::FromStr;

use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};

use crate::repositoryes::TransactionSRepository;

use super::{Slot, Subjects, Task, Transaction, User};

impl Task {
    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.id).unwrap()
    }

    pub async fn owner(&self) -> User {
        User::from_username(self.owner_username(), None)
            .await
            .unwrap()
    }

    pub fn owner_username(&self) -> String {
        self.owner_username.clone()
    }

    pub fn task(&self) -> String {
        self.task.clone()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn subject(&self) -> Subjects {
        self.subject
    }

    pub async fn confrim(self, slot: Slot) -> Result<Transaction, ()> {
        TransactionSRepository::register_transaction(&slot, self).await
    }
}

impl FromRow<'_, PgRow> for Task {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Task {
            id: row.get::<Uuid, &str>("id").to_string(),
            owner_username: row.get::<Uuid, &str>("owner").to_string(),
            task: row.get::<String, &str>("task"),
            price: row.get::<f64, &str>("price"),
            subject: Subjects::from(row.get::<&str, &str>("subject")),
            publish_time: row.get::<NaiveDateTime, &str>("publish_time"),
            target_finishing_time: row.get::<NaiveDateTime, &str>("target_finishing_time"),
        })
    }
}
