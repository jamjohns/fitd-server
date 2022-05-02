use crate::EntityId;
use crate::repository::entity::HasId;

pub struct InMemoryRepository<T> {
    entities: Vec<T>,
}

impl<T> InMemoryRepository<T> {
    pub fn new() -> InMemoryRepository<T> {
        InMemoryRepository { entities: Vec::<T>::new() }
    }
}

pub trait Repository<T> {
    fn save(&mut self, entity: T);
    fn find_by_id(&self, id: EntityId) -> Option<&T>;
    fn delete_by_id(&mut self, id: EntityId);
    fn exists_by_id(&self, id: EntityId) -> bool;
    fn count(&self) -> usize;
}

impl<T: HasId> Repository<T> for InMemoryRepository<T> {
    fn save(&mut self, entity: T) {
        self.entities.push(entity)
    }

    fn find_by_id(&self, id: EntityId) -> Option<&T> {
        self.entities.iter().find(|&e| e.id() == id)
    }

    fn delete_by_id(&mut self, id: EntityId) {
        self.entities.remove(self.entities.iter().position(|e| e.id() == id).expect(&format!("Expected {:?} -- not found!", id)));
    }

    fn exists_by_id(&self, id: EntityId) -> bool {
        self.entities.iter().find(|&e| e.id() == id).is_some()
    }

    fn count(&self) -> usize {
        self.entities.len()
    }
}