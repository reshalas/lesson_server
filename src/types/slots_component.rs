use super::{dto::SlotDTO, Slot, SlotsComponent, Subjects, Task, Transaction, User};
use crate::repositoryes::*;

impl SlotsComponent {
    pub async fn owner(&self) -> User {
        User::from_username(self.owner_username.clone(), None)
            .await
            .unwrap()
    }

    pub async fn new(owner: &User) -> SlotsComponent {
        let mut slots_component = SlotsComponent {
            owner_username: owner.username(),
            slots: Vec::new(),
        };
        slots_component.refresh().await;
        slots_component
    }

    pub fn slots(&self) -> Vec<Slot> {
        self.slots.clone()
    }

    async fn refresh(&mut self) {
        let owner = UserSRepository::from_username(self.owner_username.clone(), None)
            .await
            .unwrap();
        self.slots = SlotSRepository::active_by_owner(&owner).await
    }

    pub fn get_slot(&self, subject: Subjects) -> Option<Slot> {
        for slot in self.slots() {
            if slot.subject() == subject {
                return Some(slot);
            }
        }
        None
    }

    pub async fn add_slot(&mut self, dto: SlotDTO) -> Result<Slot, ()> {
        if self.is_slot_active(dto.subject) {
            return Err(());
        }
        let slot = if self.is_slot_in_db(dto.subject).await {
            self.activate_slot(dto).await
        } else {
            self.create_slot(dto).await
        };
        self.refresh().await;
        Ok(slot)
    }

    fn is_slot_active(&mut self, subject: Subjects) -> bool {
        for slot in self.slots() {
            if slot.subject() == subject {
                return true;
            }
        }
        false
    }

    async fn is_slot_in_db(&self, subject: Subjects) -> bool {
        SlotSRepository::exist(&self.owner().await, subject).await
    }

    async fn create_slot(&self, dto: SlotDTO) -> Slot {
        SlotSRepository::register(&self.owner().await, dto)
            .await
            .unwrap()
    }

    async fn activate_slot(&mut self, dto: SlotDTO) -> Slot {
        let mut slot = SlotSRepository::from_subject(&self.owner().await, dto.subject)
            .await
            .unwrap();
        slot.activate().await;
        slot.update_limit(dto.limit).await;
        slot
    }

    pub async fn remove_slot(&mut self, subject: Subjects) -> Result<(), ()> {
        for slot in self.slots() {
            if slot.subject() == subject {
                slot.deactivate().await?;
                self.refresh().await;
                return Ok(());
            }
        }
        return Err(());
    }

    pub async fn accept_task(&mut self, task: Task) -> Result<Transaction, String> {
        if let Some(mut slot) = self.get_slot(task.subject()) {
            if let Ok(transaction) = slot.accept_task(task).await {
                return Ok(transaction);
            }
            return Err("Task  can't be allowed".to_string());
        }
        Err("Slot doesn't exists".to_string())
    }

    pub async fn find_availible_tasks(&self) -> Vec<Task> {
        let mut tasks = Vec::new();
        for slot in self.slots() {
            tasks.append(slot.find_availible().await.as_mut());
        }
        tasks.sort_by(|task1: &Task, task2: &Task| {
            task1.publish_time.partial_cmp(&task2.publish_time).unwrap()
        });
        tasks
    }
}
