#[cfg(test)]
mod repository_tests {
    use crate::Account;
    use crate::repository::memory_repository::{InMemoryRepository};
    use crate::repository::Repository;

    #[test]
    fn create_empty_test() {
        let repository = InMemoryRepository::<Account>::new();
        assert_eq!(repository.count(), 0);
    }

    #[test]
    fn save_test() {
        let mut repository = InMemoryRepository::<Account>::new();
        assert_eq!(repository.count(), 0);
        repository.save(Account::new(0, "Steve".to_string(), "sh*sh.com".to_string()));
        assert_eq!(repository.count(), 1);
    }

    #[test]
    fn delete_test() {
        let mut repository = InMemoryRepository::<Account>::new();
        assert_eq!(repository.count(), 0);
        repository.save(Account::new(0, "Steve".to_string(), "sh*sh.com".to_string()));
        repository.save(Account::new(1, "Steve".to_string(), "sh*sh.com".to_string()));
        assert_eq!(repository.count(), 2);
        repository.delete_by_id(1);
        assert_eq!(repository.count(), 1);
    }

    #[test]
    fn exists_test() {
        let mut repository = InMemoryRepository::<Account>::new();
        assert_eq!(repository.exists_by_id(0), false);
        repository.save(Account::new(0, "Steve".to_string(), "sh*sh.com".to_string()));
        assert_eq!(repository.exists_by_id(0), true);
    }
}