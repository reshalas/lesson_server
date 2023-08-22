use sqlx::{prelude::*, Pool};
use sqlx::{types::Uuid, PgPool};

pub mod slot_s_repo;
pub mod task_s_repo;
pub mod transaction_s_repo;
pub mod user_s_repo;

static mut DB: Option<PgPool> = None;

pub struct UserSRepository(());

pub struct SlotSRepository(());

pub struct TransactionSRepository(());

pub struct TaskSRepository(());

pub fn init(pool: PgPool) {
    unsafe { DB = Some(pool) }
}

async fn generate_uuid() -> Uuid {
    unsafe {
        sqlx::query("select uuid_generate_v4();")
            .fetch_one(&DB.clone().unwrap())
            .await
            .expect("Bad request")
            .get::<Uuid, &str>("uuid_generate_v4")
    }
}

fn db() -> Pool<sqlx::Postgres> {
    unsafe { DB.clone().unwrap() }
}
