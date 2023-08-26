use super::{
    dto::{SingDto, UserDTO},
    *,
};
use crate::repositoryes::*;
use sqlx::{postgres::PgRow, prelude::*, FromRow};

impl User {
    async fn refresh(&mut self) {
        *self = UserSRepository::get_anyway(&SingDto {
            username: self.username(),
            password: self.password.clone(),
        })
        .await
        .unwrap();
        self.slots_component = SlotsComponent::new(&self).await;
        self.tasks_component = TasksComponent::new(&self).await;
    }
    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }

    pub async fn register(user: UserDTO) -> Result<(User, String), ()> {
        UserSRepository::register(&user).await
    }

    pub async fn from_dto(user: SingDto) -> Option<User> {
        let user = UserSRepository::get(&user).await;
        if let Some(mut user) = user {
            user.refresh().await;
            Some(user)
        } else {
            None
        }
    }

    pub async fn from_username(username: String, active: Option<bool>) -> Option<User> {
        let user = UserSRepository::from_username(username, active).await;
        if let Some(mut user) = user {
            user.refresh().await;
            Some(user)
        } else {
            None
        }
    }

    pub fn first_name(&self) -> &str {
        self.frirst_name.as_ref()
    }

    pub fn last_name(&self) -> &str {
        self.last_name.as_ref()
    }

    pub fn class(&self) -> i16 {
        self.class
    }

    pub fn school(&self) -> i32 {
        self.school
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn slots_component(&mut self) -> &mut SlotsComponent {
        &mut self.slots_component
    }

    pub fn tasks_component(&mut self) -> &mut TasksComponent {
        &mut self.tasks_component
    }

    pub fn location_data(&self) -> LocationData {
        self.location_data.clone()
    }

    pub async fn change_email(&mut self, email: String) {
        self.email = email.clone();
        UserSRepository::update_email(&self, email)
            .await
            .expect("Bad uuid");
    }

    pub async fn rate(&mut self, mark: i16) {
        UserSRepository::rate(&self, mark).await;
        self.raiting.push(mark);
    }
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let user = User {
            username: row.get::<String, &str>("username"),
            password: row.get::<String, &str>("password"),
            last_name: row.get::<String, &str>("last_name"),
            frirst_name: row.get::<String, &str>("first_name"),
            class: row.get::<i16, &str>("class"),
            school: row.get::<i32, &str>("school"),
            email: row.get::<String, &str>("email"),
            slots_component: SlotsComponent::default(),
            raiting: row.get::<Vec<i16>, &str>("raiting"),
            location_data: super::LocationData {
                country: row.get::<String, &str>("country"),
                city: row.get::<String, &str>("city"),
            },
            tasks_component: TasksComponent::default(),
        };
        Ok(user)
    }
}
