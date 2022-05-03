use crate::{Account, Repository};
use crate::traits::create_account::MutableCommand;
use crate::traits::has_id::HasId;

#[derive(Debug)]
pub struct CreateAccount {
    id: u32,
    name: String,
    email: String
}

impl CreateAccount {
    pub fn new(id: u32, name: String, email: String) -> CreateAccount { CreateAccount { id, email, name }}
    pub fn to_obj(&self) -> Account {Account::new(self.id, self.email.to_string(), self.name.to_string())}
}


impl MutableCommand for CreateAccount {
    fn apply<T: HasId>(&self, repository: &mut impl Repository<T>, obj: T) { repository.save(obj) }
    fn undo<T: HasId>(&self, repository: &mut impl Repository<T>) { repository.delete_by_id(self.id) }
}