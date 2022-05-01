use crate::repository::{Entity, EntityId, InMemoryRepository, Repository};

mod repository;

const NAME: &str = "Steve";

fn main() {
    println!("{}", repository::repository::say_hello(NAME));
    let mut repo = InMemoryRepository::new();
    repo.save(Entity::new(EntityId{id: 1}));
    let option_entity = repo.find_by_id(EntityId {id: 1});
    println!("{:?}", option_entity.is_some());
}
