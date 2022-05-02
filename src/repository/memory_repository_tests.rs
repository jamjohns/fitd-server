#[cfg(test)]
mod repository_tests {
    use crate::{Entity, EntityId};
    use crate::repository::memory_repository::{InMemoryRepository, Repository};

    #[test]
    fn create_empty_test() {
        let repository = InMemoryRepository::<Entity>::new();
        assert_eq!(repository.count(), 0);
    }

    #[test]
    fn save_test() {
        let mut repository = InMemoryRepository::<Entity>::new();
        assert_eq!(repository.count(), 0);
        repository.save(Entity::new(EntityId{id: 1}));
        assert_eq!(repository.count(), 1);
    }

    #[test]
    fn delete_test() {
        let mut repository = InMemoryRepository::<Entity>::new();
        assert_eq!(repository.count(), 0);
        let entity = Entity::new(EntityId{id: 1});
        repository.save(entity);
        assert_eq!(repository.count(), 1);
        repository.delete_by_id(EntityId{id:1});
        assert_eq!(repository.count(), 0);
    }

    #[test]
    fn exists_test() {
        let mut repository = InMemoryRepository::<Entity>::new();
        let id = EntityId{id: 1};
        assert_eq!(repository.exists_by_id(id), false);
        let entity = Entity::new(id);
        repository.save(entity);
        assert_eq!(repository.exists_by_id(id), true);
    }
}