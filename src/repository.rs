pub mod memory_repository;
mod memory_repository_tests;

pub trait Repository<T> {
    fn save(&mut self, entity: T);
    fn find_all(&self) -> &Vec<T>;
    fn find_by_id(&self, id: u32) -> Option<&T>;
    fn delete_by_id(&mut self, id: u32);
    fn exists_by_id(&self, id: u32) -> bool;
    fn count(&self) -> usize;
}