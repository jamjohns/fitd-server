use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use crate::character::character_placeholder::CharacterPlaceholder;
use crate::character::vice::Vice;
use crate::HasId;

#[derive(Debug)]
pub struct PlayerCharacter {
    account_id: u32,
    character_id: u32,
    playbook: String,
    heritage: String,
    background: String,
    special_ability: String,
    close_friend: String,
    rival: String,
    vice: String,
    name: String,
    alias: String,
    evocative_words: Vec<String>
}

impl PlayerCharacter {
    pub fn from_placeholder(placeholder: &CharacterPlaceholder) -> Option<PlayerCharacter> {
        if !placeholder.validated() {
            return None
        }

        Some(PlayerCharacter {
            account_id: placeholder.account_id(),
            character_id: placeholder.character_id(),
            playbook: placeholder.playbook().borrow_mut().clone().unwrap(),
            heritage: placeholder.heritage().borrow_mut().clone().unwrap(),
            background: placeholder.background().borrow_mut().clone().unwrap(),
            special_ability: placeholder.special_ability().borrow_mut().clone().unwrap(),
            close_friend: placeholder.close_friend().borrow_mut().clone().unwrap(),
            rival: placeholder.rival().borrow_mut().clone().unwrap(),
            vice: placeholder.vice().borrow_mut().clone().unwrap(),
            name: placeholder.name().borrow_mut().clone().unwrap(),
            alias: placeholder.alias().borrow_mut().clone().unwrap(),
            evocative_words: placeholder.evocative_words().to_owned()
        })
    }
}

impl HasId for PlayerCharacter {
    fn id(&self) -> u32 {
        self.character_id
    }
}