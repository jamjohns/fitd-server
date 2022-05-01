use std::borrow::{Borrow, BorrowMut};

pub mod repository {
    pub fn say_hello(name: &str) -> String {
        format!("Hello {}!", name)
    }
}

pub struct InMemoryRepository {
    entities: Vec<Entity>,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository { entities: Vec::<Entity>::new() }
    }
}

#[derive(Debug)]

pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new(id: EntityId) -> Entity {
        Entity {id}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityId {
    pub id: u8,
}

impl PartialEq for EntityId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait HasId {
    fn id(&self) -> EntityId;
}

impl HasId for Entity {
    fn id(&self) -> EntityId {
        return self.id;
    }
}

pub trait Repository {
    fn save(&mut self, entity: Entity);
    fn find_by_id(&self, id: EntityId) -> Option<&mut Entity>;
    fn delete_by_id(id: EntityId) -> bool;
    fn exists_by_id(id: EntityId) -> bool;
    fn count() -> usize;
}

impl Repository for InMemoryRepository {
    fn save(&mut self, entity: Entity) {
        self.entities.push(entity)
    }

    fn find_by_id(&self, id: EntityId) -> Option<&mut Entity> {
        for e in self.entities.as_slice() {
            println!("{:?}", e);
        }

        if let Some(i) = self.entities.iter().find(|&x| x.id == id) {}
        None
    }

    fn delete_by_id(id: EntityId) -> bool {
        todo!()
    }

    fn exists_by_id(id: EntityId) -> bool {
        todo!()
    }

    fn count() -> usize {
        todo!()
    }
}