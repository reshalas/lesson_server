use crate::types::{Slot, Task, Transaction, User};

use super::*;

impl TransactionSRepository {
    //Порождающие
    pub async fn get(id: Uuid) -> Option<Transaction> {
        match sqlx::query_as::<_, Transaction>("select * from transactions where id=$1;")
            .bind(id)
            .fetch_one(&db())
            .await
        {
            Ok(slot) => Some(slot),
            Err(_) => None,
        }
    }

    pub async fn all_by_sender_slot(sender: &Slot) -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>("select * from transactions where sender_slot=$1;")
            .bind(sender.uuid())
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    pub async fn finished_by_sender_slot(sender: &Slot) -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>(
            "select * from transactions where sender_slot=$1 and passed=true;",
        )
        .bind(sender.uuid())
        .fetch_all(&db())
        .await
        .expect("Bad request")
    }

    pub async fn unfinished_by_sender_slot(sender: &Slot) -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>(
            "select * from transactions where sender_slot=$1 and passed=false;",
        )
        .bind(sender.uuid())
        .fetch_all(&db())
        .await
        .expect("Bad request")
    }

    pub async fn all_by_recipient(recipient: &User) -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>("select * from transactions where recipient =$1;")
            .bind(recipient.username())
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    pub async fn finished_by_recipient_id(recipient: &User) -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>(
            "select * from transactions where recipient =$1 and passed=true;",
        )
        .bind(recipient.username())
        .fetch_all(&db())
        .await
        .expect("Bad request")
    }

    pub async fn unfinished_by_reciepment(recipient: &User) -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>(
            "select * from transactions where recipient =$1 and passed=false;",
        )
        .bind(recipient.username())
        .fetch_all(&db())
        .await
        .expect("Bad request")
    }

    pub async fn all() -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>("select * from transactions;")
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    pub async fn all_unfinished() -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>("select * from transactions where passed = false;")
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    pub async fn all_finished() -> Vec<Transaction> {
        sqlx::query_as::<_, Transaction>("select * from transactions where passed = true;")
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    //Регистрация
    pub async fn register_transaction(sender: &Slot, task: Task) -> Result<Transaction, ()> {
        let id = generate_uuid().await;

        //Создаем слот
        match sqlx::query(
            "insert into transactions (id, sender_slot, recipient, passed, time_of_rigestration, date_of_rigestration, time_of_ending, date_of_ending, price, task)
             values($1, $2, $3, false, $4, $5, null, null, $6, $7);",
        )
        .bind(id)
        .bind(sender.uuid())
        .bind(task.owner_username())
        .bind(chrono::Utc::now().time())
        .bind(chrono::Utc::now().date_naive())
        .bind(task.price())
        .bind(task.task())
        .execute(&db())
        .await
        {
            Ok(_) => {
                TaskSRepository::delete(task).await?;
                Ok(TransactionSRepository::get(id).await.expect("Bad uuid"))
            }
            Err(_) => Err(()),
        }
    }

    pub async fn finish_transactions(transaction: Transaction) -> Result<Transaction, ()> {
        match sqlx::query("update transactions set passed=true, time_of_ending=$1, date_of_ending=$2 where id = $3;")
            .bind(chrono::Utc::now().time())
            .bind(chrono::Utc::now().date_naive())
            .bind(transaction.uuid())
            .execute(&db())
            .await
        {
            Ok(_) => Ok(TransactionSRepository::get(transaction.uuid()).await.expect("Bad uuid")),
            Err(_) => Err(()),
        }
    }
}
