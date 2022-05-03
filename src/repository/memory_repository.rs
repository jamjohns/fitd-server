use crate::repository::Repository;
use crate::traits::has_id::HasId;

pub struct InMemoryRepository<T> {
    entities: Vec<T>,
}

impl<T: HasId> InMemoryRepository<T> {
    pub fn new() -> InMemoryRepository<T> {
        InMemoryRepository { entities: Vec::<T>::new() }
    }
}

impl<T: HasId> Repository<T> for InMemoryRepository<T> {
    fn save(&mut self, entity: T) {
        self.entities.push(entity)
    }

    fn find_all(&self) -> &Vec<T> {
        &self.entities
    }

    fn find_by_id(&self, id: u32) -> Option<&T> {
        self.entities.iter().find(|&e| e.id() == id)
    }

    fn delete_by_id(&mut self, id: u32) {
        self.entities.remove(self.entities.iter().position(|e| e.id() == id).expect(&format!("Expected {:?} -- not found!", id)));
    }

    fn exists_by_id(&self, id: u32) -> bool {
        self.entities.iter().find(|&e| e.id() == id).is_some()
    }

    fn count(&self) -> usize {
        self.entities.len()
    }
}