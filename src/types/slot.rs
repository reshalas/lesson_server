use super::*;
use crate::repositoryes::*;
use sqlx::postgres::PgRow;
use sqlx::prelude::*;
use sqlx::types::Uuid;
use sqlx::FromRow;
use std::str::FromStr;

impl Slot {
    pub async fn from_uuid(uuid: Uuid) -> Option<Slot> {
        let slot = SlotSRepository::get(uuid).await;
        match slot {
            Some(mut slot) => {
                slot.refresh().await;
                Some(slot)
            }
            None => None,
        }
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(self.id.as_str()).expect("Can't parce uuid")
    }

    pub fn limit(&self) -> Option<i32> {
        self.limit
    }

    pub async fn accept_task(&mut self, task: Task) -> Result<Transaction, ()> {
        if !self.can_allow_transaction() {
            return Err(());
        }
        if task.subject() != self.subject() {
            return Err(());
        }
        let transaction = Transaction::register(&self, task).await?;
        self.refresh().await;
        Ok(transaction)
    }

    pub fn can_allow_transaction(&self) -> bool {
        if let Some(limit) = self.limit() {
            let transactions = self.pending_transactions.clone();
            return (transactions.len() as i32) < limit;
        }
        true
    }

    pub async fn update_limit(&mut self, limit: Option<i32>) {
        self.limit = limit;
        SlotSRepository::update_transaction_limit(&self, limit)
            .await
            .expect("Bad request");
    }

    pub fn subject(&self) -> Subjects {
        self.subject.clone()
    }

    pub async fn activate(&self) {
        SlotSRepository::activate(&self).await.unwrap();
    }

    pub async fn deactivate(self) -> Result<(), ()> {
        if !self.can_be_deactivated() {
            return Err(());
        }
        SlotSRepository::deactivate(self).await.unwrap();
        Ok(())
    }

    pub fn pending_transactions(&self) -> Vec<Transaction> {
        self.pending_transactions.clone()
    }

    pub fn can_be_deactivated(&self) -> bool {
        self.pending_transactions().is_empty()
    }

    pub async fn owner(&self) -> User {
        User::from_username(self.owner_username.clone(), None)
            .await
            .unwrap()
    }

    async fn refresh(&mut self) {
        self.pending_transactions = TransactionSRepository::unfinished_by_sender_slot(self).await
    }

    pub async fn find_availible(&self) -> Vec<Task> {
        let mut tasks = Vec::new();
        let mut users = UserSRepository::find_coschoollers(&self.owner().await).await;
        for user in &mut users {
            tasks.append(user.tasks_component().tasks(self.subject()).as_mut());
        }
        tasks
    }
}

impl FromRow<'_, PgRow> for Slot {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Slot {
            id: row.get::<Uuid, &str>("id").to_string(),
            owner_username: row.get::<String, &str>("owner"),
            subject: Subjects::from(row.get::<&str, &str>("lesson")),
            limit: row.get::<Option<i32>, &str>("transactions_limit"),
            pending_transactions: Vec::new(),
        })
    }
}
