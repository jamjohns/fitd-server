use crate::character::vice::Vice;

#[derive(Debug)]
pub struct CharacterPlaceholder {
    account_id: u32,
    character_id: u32,
    validated: bool,
    playbook: Option<String>,
    heritage: Option<String>,
    background: Option<String>,
    special_ability: Option<String>,
    close_friend: Option<String>,
    rival: Option<String>,
    vice: Option<String>,
    name: Option<String>,
    alias: Option<String>,
    evocative_words: Vec<String>
}

impl CharacterPlaceholder {
    pub fn new(account_id: u32, character_id: u32) -> CharacterPlaceholder {
        CharacterPlaceholder {
            account_id,
            character_id,
            validated: false,
            playbook: None,
            heritage: None,
            background: None,
            special_ability: None,
            close_friend: None,
            rival: None,
            vice: None,
            name: None,
            alias: None,
            evocative_words: vec![]
        }
    }

    pub fn validated(&self) -> bool {
        self.validated
    }

    pub fn account_id(&self) -> u32 {
        self.account_id
    }

    pub fn character_id(&self) -> u32 {
        self.character_id
    }

    pub fn playbook(&self) -> &Option<String> {
        &self.playbook
    }

    pub fn set_playbook(&mut self, playbook: String) {
        self.playbook = Some(playbook)
    }

    pub fn heritage(&self) -> &Option<String> {
        &self.heritage
    }

    pub fn set_heritage(&mut self, heritage: String) {
        self.heritage = Some(heritage)
    }

    pub fn background(&self) -> &Option<String> {
        &self.background
    }

    pub fn set_background(&mut self, background: String) {
        self.background = Some(background)
    }

    pub fn special_ability(&self) -> &Option<String> {
        &self.special_ability
    }

    pub fn set_special_ability(&mut self, special_ability: String) {
        self.special_ability = Some(special_ability)
    }

    pub fn close_friend(&self) -> &Option<String> {
        &self.close_friend
    }

    pub fn set_close_friend(&mut self, close_friend: String) {
        self.close_friend = Some(close_friend)
    }

    pub fn rival(&self) -> &Option<String> {
        &self.rival
    }

    pub fn set_rival(&mut self, rival: String) {
        self.rival = Some(rival)
    }

    pub fn vice(&self) -> &Option<String> {
        &self.vice
    }

    pub fn set_vice(&mut self, vice: String) {
        self.vice = Some(vice)
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn alias(&self) -> &Option<String> {
        &self.alias
    }

    pub fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias)
    }

    pub fn evocative_words(&self) -> &Vec<String> {
        &self.evocative_words
    }

    pub fn add_evocative_word(&mut self, word: String) {
        self.evocative_words.push(word)
    }

    pub fn validate(&mut self) {
        if self.name.is_some() {
            println!("Checking name...");
            self.validated = true;
            println!("Validated!");
        }
    }
}