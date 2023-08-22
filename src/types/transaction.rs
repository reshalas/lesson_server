use super::*;
use crate::repositoryes::{SlotSRepository, TransactionSRepository};
use chrono::{NaiveDate, NaiveTime};
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};
use std::str::FromStr;

impl Transaction {
    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(self.id.as_str()).expect("Can't parce uuid")
    }

    pub async fn sender_slot(&self) -> Slot {
        SlotSRepository::get(self.sender_slot_uuid()).await.unwrap()
    }

    pub async fn sender(&self) -> User {
        self.sender_slot().await.owner().await
    }

    pub fn sender_slot_uuid(&self) -> Uuid {
        Uuid::from_str(self.sender_slot.as_str()).expect("Can't parce uuid")
    }

    pub async fn recipient(&self) -> User {
        User::from_username(self.recipient_username.clone(), None)
            .await
            .unwrap()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn passed(&self) -> bool {
        self.passed
    }

    pub fn time_of_rigestration(&self) -> NaiveTime {
        self.time_of_rigestration
    }

    pub fn date_of_rigestration(&self) -> NaiveDate {
        self.date_of_rigestration
    }

    pub fn time_of_ending(&self) -> Option<NaiveTime> {
        self.time_of_ending
    }

    pub fn date_of_ending(&self) -> Option<NaiveDate> {
        self.date_of_ending
    }

    pub async fn register(sender: &Slot, task: Task) -> Result<Transaction, ()> {
        TransactionSRepository::register_transaction(&sender, task).await
    }

    pub async fn finish(self) {
        TransactionSRepository::finish_transactions(self)
            .await
            .expect("Bad uuid");
    }
}

impl FromRow<'_, PgRow> for Transaction {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Transaction {
            id: row.get::<Uuid, &str>("id").to_string(),
            sender_slot: row.get::<Uuid, &str>("sender_slot").to_string(),
            recipient_username: row.get::<Uuid, &str>("recipient").to_string(),
            price: row.get::<f64, &str>("price"),
            passed: row.get::<bool, &str>("passed"),
            task: row.get::<String, &str>("task"),
            time_of_rigestration: row.get::<NaiveTime, &str>("time_of_rigestration"),
            date_of_rigestration: row.get::<NaiveDate, &str>("date_of_rigestration"),
            time_of_ending: row.get::<Option<NaiveTime>, &str>("time_of_ending"),
            date_of_ending: row.get::<Option<NaiveDate>, &str>("date_of_ending"),
        })
    }
}
