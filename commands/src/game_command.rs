use bevy::prelude::{Entity, World};

pub trait GameCommand {
    fn name(&self) -> String;
    fn can_execute(&self, command: Vec<String>, acting_entity: &Entity, world: &World) -> bool;
    fn execute(
        &self,
        command: Vec<String>,
        acting_entity: &Entity,
        world: &mut World,
    ) -> Result<(), String>;
}
