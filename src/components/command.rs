use crate::id::command::Command;
use amethyst::ecs::{Component, DenseVecStorage};
use std::collections::HashSet;

pub struct ActiveCommand {
    active_commands: HashSet<Command>,
}

impl ActiveCommand {
    pub fn activate(&mut self, command: Command) {
        self.active_commands.insert(command);
    }

    pub fn active_commands(&self) -> impl Iterator<Item = &Command> {
        self.active_commands.iter()
    }
}

impl Component for ActiveCommand {
    type Storage = DenseVecStorage<Self>;
}
