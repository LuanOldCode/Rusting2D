pub struct Entity {
    pub id: u32,
    pub name: String,
}

pub struct EntityManager {
    entities: Vec<Entity>,
    next_id: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(Entity {
            id: self.next_id,
            name: entity.name,
        });
        self.next_id += 1;
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
