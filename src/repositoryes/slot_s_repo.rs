use super::*;
use crate::types::{dto::SlotDTO, Slot, Subjects, User};
use sqlx::types::Uuid;

impl SlotSRepository {
    pub async fn get(id: Uuid) -> Option<Slot> {
        match sqlx::query_as::<_, Slot>("select * from slots where id=$1;")
            .bind(id)
            .fetch_one(&db())
            .await
        {
            Ok(slot) => Some(slot),
            Err(_) => None,
        }
    }

    pub async fn from_subject(owner: &User, subject: Subjects) -> Option<Slot> {
        match sqlx::query_as::<_, Slot>("select * from slots where owner=$1 and lesson = $2;")
            .bind(owner.username())
            .bind(subject.to_string())
            .fetch_one(&db())
            .await
        {
            Ok(slot) => Slot::from_uuid(slot.uuid()).await,
            Err(_) => None,
        }
    }

    pub async fn from_owner(owner: &User) -> Vec<Slot> {
        let mut slots = sqlx::query_as::<_, Slot>("select * from slots where owner=$1;")
            .bind(owner.username())
            .fetch_all(&db())
            .await
            .expect("Bad request");
        for slot in &mut slots {
            *slot = Slot::from_uuid(slot.uuid()).await.unwrap();
        }
        slots
    }

    pub async fn active_by_owner(owner: &User) -> Vec<Slot> {
        let mut slots =
            sqlx::query_as::<_, Slot>("select * from slots where owner=$1 and is_active = true;")
                .bind(owner.username())
                .fetch_all(&db())
                .await
                .expect("Bad request");
        for slot in &mut slots {
            *slot = Slot::from_uuid(slot.uuid()).await.unwrap();
        }
        slots
    }

    pub async fn all() -> Vec<Slot> {
        let mut slots = sqlx::query_as::<_, Slot>("select * from slots;")
            .fetch_all(&db())
            .await
            .expect("Bad request");
        for slot in &mut slots {
            *slot = Slot::from_uuid(slot.uuid()).await.unwrap();
        }
        slots
    }

    pub async fn exist(user: &User, subject: Subjects) -> bool {
        let slots = SlotSRepository::from_owner(&user).await;
        for slot in slots {
            if slot.subject() == subject {
                return true;
            }
        }
        false
    }

    pub async fn update_transaction_limit(slot: &Slot, new_limit: Option<i32>) -> Result<(), ()> {
        match sqlx::query("update slots set transactions_limit=$1 where id=$2;")
            .bind(new_limit)
            .bind(slot.uuid())
            .execute(&db())
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    //Удаление
    pub async fn deactivate(slot: Slot) -> Result<(), ()> {
        match sqlx::query("update slots set is_active = false where id=$1;")
            .bind(slot.uuid())
            .execute(&db())
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub async fn activate(slot: &Slot) -> Result<Slot, ()> {
        match sqlx::query("update slots set is_active = true where id=$1;")
            .bind(slot.uuid())
            .execute(&db())
            .await
        {
            Ok(_) => Ok(SlotSRepository::get(slot.uuid()).await.unwrap()),
            Err(_) => Err(()),
        }
    }

    //Регистрация
    pub async fn register(owner: &User, new_slot: SlotDTO) -> Result<Slot, ()> {
        let id = generate_uuid().await;
        //Создаем слот
        match sqlx::query("insert into slots (id, owner, lesson, transactions_limit, is_active) values($1, $2, $3, $4, true);")
            .bind(id)
            .bind(owner.username())
            .bind(new_slot.subject.to_string())
            .bind(new_slot.limit)
            .execute(&db())
            .await
        {
            Ok(_) => Ok(SlotSRepository::get(id).await.expect("Bad uuid")),
            Err(_) => Err(()),
        }
    }
}
