use std::borrow::Borrow;
use crate::traits::has_id::HasId;

#[derive(Debug)]
pub struct Account {
    id: u32,
    email: String,
    name: String
}

impl Account {
    pub fn new(id: u32, email: String, name: String) -> Account { Account { id, email, name }}
    pub fn email(&self) -> &str {self.email.borrow()}
    pub fn name(&self) -> &str {self.name.borrow()}
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}

impl HasId for Account {
    fn id(&self) -> u32 {
        self.id
    }
}