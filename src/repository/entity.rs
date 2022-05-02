#[derive(Debug)]
pub struct Entity {
    id: EntityId,
}

#[derive(Copy, Clone, Debug)]
pub struct EntityId {
    pub id: u8,
}

impl Entity {
    pub fn new(id: EntityId) -> Entity {
        Entity {id}
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for EntityId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait HasId {
    fn id(&self) -> EntityId;
}

impl HasId for Entity {
    fn id(&self) -> EntityId {
        return self.id;
    }
}