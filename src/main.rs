use std::borrow::{Borrow, BorrowMut};
use std::time::Instant;
use crate::character::character_placeholder::CharacterPlaceholder;
use crate::character::player_character::PlayerCharacter;
use crate::character::vice::Vice;
use crate::model::account::Account;
use crate::repository::memory_repository::{InMemoryRepository};
use crate::repository::Repository;
use crate::command::create_account::CreateAccount;
use crate::traits::create_account::MutableCommand;
use crate::traits::has_id::HasId;

mod repository;
mod model;
mod traits;
mod command;
mod character;

fn main() {
    // instantiate repositories
    let account_repo = &mut InMemoryRepository::<Account>::new();
    let player_character_repo = &mut InMemoryRepository::<PlayerCharacter>::new();

    // instantiate commands

    let create_steve_account_command = CreateAccount::new(1, "Steve".to_string(), "Steve@Steve.com".to_string());
    let create_james_account_command = CreateAccount::new(2, "James".to_string(), "James@James.com".to_string());

    // command knows how to create account and add it to repo
    // TODO make apply static?

    create_steve_account_command.apply(account_repo, create_steve_account_command.to_obj());
    create_james_account_command.apply(account_repo, create_james_account_command.to_obj());

    // verify we can retrieve accounts from repo
    // repo returns an option since this can fail

    let steve_account_option = account_repo.find_by_id(1);
    let james_account_option = account_repo.find_by_id(2);

    // if the accounts were retrieved...
    if james_account_option.is_some() && steve_account_option.is_some() {

        // unwrap converts the Optional to the actual
        let james_account = james_account_option.unwrap();
        let steve_account = steve_account_option.unwrap();

        // build a character placeholder; this encapsulates fluxuating state
        let mut phoenix = build_phoenix(james_account.id());
        let mut barltok = build_barltok(steve_account.id());

        // once the player/DM thinks its ready; validate it (this just sets a bool)
        validate(phoenix.borrow_mut());
        validate(barltok.borrow_mut());

        // this shouldn't ever fail because validate should catch it
        let phoenix_player_character = PlayerCharacter::from_placeholder(&phoenix).unwrap();
        let barltok_player_character = PlayerCharacter::from_placeholder(&barltok).unwrap();

        // persist
        player_character_repo.save(phoenix_player_character);
        player_character_repo.save(barltok_player_character);
    }

    // lets check the PlayerCharacter repository

    let player_characters = player_character_repo.find_all();

    for player in player_characters {
        println!("{:#?}", player);
    }
}

fn validate(placeholder: &mut CharacterPlaceholder) {
    placeholder.validate();
}

fn character(placeholder: &CharacterPlaceholder) -> Option<PlayerCharacter> {
    PlayerCharacter::from_placeholder(placeholder)
}

fn build_barltok(account_id: u32) -> CharacterPlaceholder {
    let mut placeholder = CharacterPlaceholder::new(account_id, 1);
    placeholder.set_alias("Pawnman".to_string());
    placeholder.set_background("Broke as shit.".to_string());
    placeholder.set_name("Barltok".to_string());
    placeholder.set_heritage("Grew up in a pawn shop.".to_string());
    placeholder.set_rival("Guy who owns 'Central Pawn'.".to_string());
    placeholder.set_close_friend("Guy who owns 'Pawn Central".to_string());
    placeholder.set_playbook("Pawn Employee".to_string());
    placeholder.set_special_ability("Exchange rare magical items for coppers.".to_string());
    placeholder.set_vice("WEIRD".to_string());
    placeholder.add_evocative_word("Best".to_string());
    placeholder.add_evocative_word("I".to_string());
    placeholder.add_evocative_word("Can".to_string());
    placeholder.add_evocative_word("Do".to_string());
    placeholder.add_evocative_word("Is...".to_string());
    placeholder
}

fn build_phoenix(account_id: u32) -> CharacterPlaceholder {
    let mut placeholder = CharacterPlaceholder::new(account_id, 1);
    placeholder.set_alias("Beardedman".to_string());
    placeholder.set_background("Grew up in Hovie Jovie".to_string());
    placeholder.set_name("Phoenix".to_string());
    placeholder.set_heritage("Progeny of a long line of...".to_string());
    placeholder.set_rival("Guy who thinks he might be Jesus.".to_string());
    placeholder.set_close_friend("Guy who wonders if Eve fucked the snake.".to_string());
    placeholder.set_playbook("Argument".to_string());
    placeholder.set_special_ability("Research".to_string());
    placeholder.set_vice("FAITH".to_string());
    placeholder.add_evocative_word("You".to_string());
    placeholder.add_evocative_word("Are".to_string());
    placeholder.add_evocative_word("Not".to_string());
    placeholder.add_evocative_word("Jesus".to_string());
    placeholder
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