use crate::repository::entity::{Entity, EntityId};
use crate::repository::memory_repository::{InMemoryRepository, Repository};

mod repository;

fn main() {
    let mut repo = InMemoryRepository::<Entity>::new();
    repo.save(Entity::new(EntityId{id: 1}));
    let option_entity = repo.find_by_id(EntityId {id: 1});
    println!("{:?}", option_entity.is_some());
}
