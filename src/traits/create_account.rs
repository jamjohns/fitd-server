use crate::Repository;
use crate::traits::has_id::HasId;

pub trait MutableCommand {
    fn apply<T: HasId>(&self, repository: &mut impl Repository<T>, obj: T);
    fn undo<T: HasId>(&self, repository: &mut impl Repository<T>);
}