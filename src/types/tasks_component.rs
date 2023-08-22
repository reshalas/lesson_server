use super::{dto::TaskDTO, *};
use crate::repositoryes::*;

impl TasksComponent {
    pub async fn owner(&self) -> User {
        User::from_username(self.owner_username.clone(), None)
            .await
            .unwrap()
    }

    pub async fn new(owner: &User) -> TasksComponent {
        let mut tasks_component = TasksComponent {
            owner_username: owner.username(),
            tasks: Vec::new(),
        };
        tasks_component.refresh().await;
        tasks_component
    }

    async fn refresh(&mut self) {
        let owner = UserSRepository::from_username(self.owner_username.clone(), None)
            .await
            .unwrap();
        self.tasks = TaskSRepository::from_owner(&owner).await
    }

    pub fn tasks(&self, subjects: Subjects) -> Vec<Task> {
        let mut tasks = Vec::new();
        for task in self.tasks.clone() {
            if task.subject == subjects {
                tasks.push(task);
            }
        }
        tasks
    }

    pub async fn publish_task(&mut self, dto: TaskDTO) -> Task {
        let task = TaskSRepository::publish(&self.owner().await, dto)
            .await
            .unwrap();
        self.refresh().await;
        task
    }
}
