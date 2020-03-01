use crate::id::command::Command as CommandId;
use amethyst::{
    assets::{Asset, Handle},
    ecs::DenseVecStorage,
};
use command_parser::Command;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type CommandListHandle = Handle<CommandList>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandList {
    #[serde(bound(deserialize = "BTreeMap<CommandId, Command>: Deserialize<'de>"))]
    commands: BTreeMap<CommandId, Command>,
}

impl Asset for CommandList {
    const NAME: &'static str = "COMMAND_LIST";

    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

impl CommandList {
    pub fn new() -> Self {
        CommandList {
            commands: BTreeMap::new(),
        }
    }

    pub fn commands(&self) -> impl Iterator<Item = (&CommandId, &Command)> {
        self.commands.iter()
    }

    pub fn command(&self, key: &CommandId) -> Option<&Command> {
        self.commands.get(key)
    }

    #[cfg(feature = "serialize")]
    pub fn add_command(&mut self, key: CommandId, command: &str) -> Result<(), failure::Error> {
        let command = Command::new(command)?;
        self.commands.insert(key, command);
        Ok(())
    }
}

pub struct CommandStore {
    command_lists: BTreeMap<String, CommandListHandle>,
}

impl CommandStore {
    pub fn new() -> Self {
        CommandStore {
            command_lists: std::collections::BTreeMap::new(),
        }
    }

    pub fn commands(&self) -> impl Iterator<Item = (&String, &CommandListHandle)> {
        self.command_lists.iter()
    }

    pub fn add_command(&mut self, key: &str, command: CommandListHandle) {
        self.command_lists.insert(key.into(), command);
    }
}
