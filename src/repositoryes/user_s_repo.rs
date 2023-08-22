use std::str::FromStr;

use super::{db, generate_uuid, UserSRepository};
use crate::types::{
    dto::{SingDto, UserDTO},
    errors::VerfyEmailError,
    User,
};
use sqlx::{postgres::PgQueryResult, types::Uuid, Error, Row};

impl UserSRepository {
    async fn fill_users_table(new_user: &UserDTO) -> Result<(), Error> {
        sqlx::query(
            "insert into users (username, password, first_name, last_name, class, school, email, raiting, is_active)
         values($1, $2, $3, $4, $5, $6, $7, '{}', false);",
        )
        .bind(new_user.username.clone())
        .bind(new_user.password.clone())
        .bind(new_user.first_name.clone())
        .bind(new_user.last_name.clone())
        .bind(new_user.class.clone())
        .bind(new_user.school.clone())
        .bind(new_user.email.clone())
        .execute(&db())
        .await?;
        Ok(())
    }

    async fn fill_location_table(new_user: &UserDTO) -> Result<PgQueryResult, Error> {
        sqlx::query("insert into location_data (owner, country, city) values($1, $2, $3);")
            .bind(new_user.username.clone())
            .bind(new_user.location_data.country.clone())
            .bind(new_user.location_data.city.clone())
            .execute(&db())
            .await
    }

    async fn fill_email_verefication_table(
        new_user: &UserDTO,
    ) -> Result<(PgQueryResult, String), Error> {
        let uuid = generate_uuid().await;
        match sqlx::query(
            "insert into email_vereficator (username, id, registration_data) values($1, $2, $3);",
        )
        .bind(new_user.username.clone())
        .bind(uuid)
        .bind(chrono::Utc::now().date_naive())
        .execute(&db())
        .await
        {
            Ok(res) => Ok((res, uuid.to_string())),
            Err(err) => Err(err),
        }
    }

    pub async fn register(new_user: &UserDTO) -> Result<(User, String), ()> {
        if let Ok(_) = UserSRepository::fill_users_table(&new_user).await {
            UserSRepository::fill_location_table(&new_user)
                .await
                .unwrap();
            let (_, uuid) = UserSRepository::fill_email_verefication_table(&new_user)
                .await
                .unwrap();
            return Ok((
                UserSRepository::from_username(new_user.username.clone(), None)
                    .await
                    .unwrap(),
                uuid,
            ));
        }
        Err(())
    }

    pub async fn verfy_email(uuid: &str, username: &str) -> Result<(), VerfyEmailError> {
        match delete_verefiacation_data(username, uuid).await {
            Ok(_) => {
                sqlx::query("update users set is_active=true where username=$1;")
                    .bind(username)
                    .execute(&db())
                    .await
                    .unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn cancel_verefication(uuid: &str, username: &str) -> Result<(), VerfyEmailError> {
        match delete_verefiacation_data(username, uuid).await {
            Ok(_) => {
                delete_user(username).await.unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get(sing_data: &SingDto) -> Option<User> {
        match UserSRepository::from_username(sing_data.username.clone(), Some(true)).await {
            Some(user) => {
                if user.password() == sing_data.password {
                    Some(user)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub async fn get_anyway(sing_data: &SingDto) -> Option<User> {
        match UserSRepository::from_username(sing_data.username.clone(), None).await {
            Some(user) => {
                if user.password() == sing_data.password {
                    Some(user)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub async fn is_username_free(username: String) -> bool {
        if let Some(_) = UserSRepository::from_username(username, None).await {
            return false;
        }
        true
    }

    pub async fn is_email_free(email: String) -> bool {
        if let Some(_) = UserSRepository::from_email(email, None).await {
            return false;
        }
        true
    }

    pub async fn from_email(email: String, active: Option<bool>) -> Option<User> {
        match sqlx::query("select username from users where email = $1;")
            .bind(email)
            .fetch_one(&db())
            .await
        {
            Ok(row) => UserSRepository::from_username(row.get("username"), active).await,
            Err(_) => None,
        }
    }

    pub async fn from_username(username: String, active: Option<bool>) -> Option<User> {
        let sql = format!(
            "select * from users join location_data on owner=username where username = $1 {};",
            match active {
                Some(val) => format!("and is_active={}", val),
                None => String::new(),
            }
        );
        match sqlx::query_as::<_, User>(&sql)
            .bind(username)
            .fetch_one(&db())
            .await
        {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn all() -> Vec<User> {
        let mut users =
            sqlx::query_as::<_, User>("select * from users join location_data on owner=username;")
                .fetch_all(&db())
                .await
                .expect("Bad request");
        for user in &mut users {
            *user = User::from_username(user.username(), None).await.unwrap();
        }
        users
    }

    pub async fn update_email(user: &User, email: String) -> Result<(), ()> {
        match sqlx::query("update users set email = $1 where username=$2;")
            .bind(email)
            .bind(user.username())
            .execute(&db())
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    //изменение
    pub async fn rate(user: &User, mark: i16) {
        sqlx::query("update users set raiting=array_append(raiting, $1) where username=$2;")
            .bind(mark)
            .bind(user.username())
            .execute(&db())
            .await
            .expect("Bad request");
    }

    pub async fn find_coschoollers(user: &User) -> Vec<User> {
        let users = sqlx::query_as::<_, User>("select * from users join location_data on owner=id where school = $1, country = $4, city = $5;")
            .bind(user.school())
            .bind(user.location_data().country)
            .bind(user.location_data().city)
            .fetch_all(&db())
            .await.unwrap();
        users
    }
}

async fn delete_user(username: &str) -> Result<PgQueryResult, Error> {
    sqlx::query("delete from location_data where owner=$1;")
        .bind(username)
        .execute(&db())
        .await?;
    sqlx::query("delete from users where username=$1;")
        .bind(username)
        .execute(&db())
        .await
}

async fn delete_verefiacation_data(username: &str, uuid: &str) -> Result<(), VerfyEmailError> {
    let uuid = Uuid::from_str(uuid).unwrap();
    if UserSRepository::is_username_free(username.to_string()).await {
        return Err(VerfyEmailError::WrongVerfyKey);
    }
    println!("a");
    if UserSRepository::from_username(username.to_string(), Some(true))
        .await
        .is_some()
    {
        return Err(VerfyEmailError::EmailAlreadyVerfied);
    }
    if let Ok(row) = sqlx::query("select * from email_vereficator where id=$1;")
        .bind(uuid)
        .fetch_one(&db())
        .await
    {
        println!("sdfgh");
        let username_from_db: String = row.get("username");
        if username != username_from_db {
            return Err(VerfyEmailError::WrongVerfyKey);
        }

        sqlx::query("delete from email_vereficator where id=$1;")
            .bind(uuid)
            .execute(&db())
            .await
            .unwrap();
        return Ok(());
    }
    println!("w");
    Err(VerfyEmailError::WrongVerfyKey)
}
