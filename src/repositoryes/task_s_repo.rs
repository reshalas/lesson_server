use super::*;
use crate::types::{dto::TaskDTO, *};
use chrono::Utc;

impl TaskSRepository {
    pub async fn all() -> Vec<Task> {
        sqlx::query_as::<_, Task>("select * from tasks;")
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    pub async fn get(id: Uuid) -> Option<Task> {
        match sqlx::query_as::<_, Task>("select * from tasks where id=$1;")
            .bind(id)
            .fetch_one(&db())
            .await
        {
            Ok(task) => Some(task),
            Err(_) => None,
        }
    }

    pub async fn from_owner(owner: &User) -> Vec<Task> {
        sqlx::query_as::<_, Task>("select * from tasks where owner=$1;")
            .bind(owner.username())
            .fetch_all(&db())
            .await
            .expect("Bad request")
    }

    pub async fn publish(owner: &User, dto: TaskDTO) -> Result<Task, ()> {
        let id = generate_uuid().await;
        match sqlx::query("insert into tasks (id, owner, task, subject, publish_time, target_finishing_time, price) values($1, $2, $3, $4, $5, $6, $7);")
            .bind(id)
            .bind(owner.username())
            .bind(dto.task)
            .bind(dto.subject.to_string())
            .bind(Utc::now())
            .bind(dto.target_finishing_time)
            .bind(dto.price)
            .execute(&db())
            .await
        {
            Ok(_) => Ok(TaskSRepository::get(id).await.expect("Bad uuid")),
            Err(_) => Err(()),
        }
    }

    pub async fn delete(task: Task) -> Result<(), ()> {
        match sqlx::query("delete from tasks where id=$1;")
            .bind(task.uuid())
            .execute(&db())
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
