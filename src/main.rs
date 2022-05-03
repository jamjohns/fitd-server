use std::time::Instant;
use crate::model::account::Account;
use crate::repository::memory_repository::{InMemoryRepository};
use crate::repository::Repository;
use crate::command::create_account::CreateAccount;
use crate::traits::create_account::MutableCommand;

mod repository;
mod model;
mod traits;
mod command;

fn main() {
    account();
}

fn account() {
    let account_repo = &mut InMemoryRepository::<Account>::new();
    let mut i:u32 = 0;
    let mut commands = Vec::new();
    let tik = Instant::now();
    while i<100000 {
        let command = CreateAccount::new(i, "Steve".to_owned(), "s@.com".to_owned());
        command.apply(account_repo, command.to_obj());
        commands.push(command);
        i += 1;
    }
    let mut tok = tik.elapsed().as_millis();
    println!("{} accounts instantiated in: {}ms", account_repo.count(), tok);
    let mut deleted = 0;
    for command in commands {
        command.undo(account_repo);
        deleted += 1;
    }
    tok = tik.elapsed().as_millis();
    println!("{} accounts deleted in: {}ms", deleted, tok);
}

fn view_repo(repo: &InMemoryRepository<Account>) {
    println!("Account Repo has ({})...", repo.count());
    for account in repo.find_all() {
        println!("{:?}", account);
    }
}