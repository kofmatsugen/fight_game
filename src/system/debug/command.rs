use crate::{
    id::command::Command,
    input::FightInput,
    resource::command::{CommandList, CommandStore},
};
use amethyst::{
    assets::AssetStorage,
    ecs::{Entity, Read, ReadExpect, ReaderId, System, World, WriteStorage},
    shrev::EventChannel,
    ui::{UiFinder, UiText},
};
use input_handle::traits::InputParser;
use std::collections::BTreeMap;

type Event<'a> = <FightInput as InputParser<'a>>::Event;

pub struct CommandDebugSystem<'a> {
    debug_commands: Option<Entity>,
    reader: ReaderId<Event<'a>>,
    last_commands: BTreeMap<Entity, Command>,
}

impl<'a> CommandDebugSystem<'a> {
    pub fn new(world: &mut World) -> Self {
        CommandDebugSystem {
            debug_commands: None,
            reader: world
                .fetch_mut::<EventChannel<Event<'a>>>()
                .register_reader(),
            last_commands: BTreeMap::new(),
        }
    }
}

impl<'a, 's> System<'s> for CommandDebugSystem<'a> {
    type SystemData = (
        UiFinder<'s>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CommandStore>,
        Read<'s, AssetStorage<CommandList>>,
        Read<'s, EventChannel<Event<'a>>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (finder, mut texts, _store, _list, channel) = data;
        self.debug_commands = finder.find("debug_command_list");
        for &(e, id) in channel.read(&mut self.reader) {
            self.last_commands
                .entry(e)
                .and_modify(|entried| {
                    if *entried != id {
                        *entried = id;
                    }
                })
                .or_insert(id);
        }
        self.debug_commands
            .map(|e| update_detected(e, &mut texts, &self.last_commands));
    }
}

fn update_detected(
    commands: Entity,
    texts: &mut WriteStorage<UiText>,
    last_commands: &BTreeMap<Entity, Command>,
) -> Option<()> {
    let commands = texts.get_mut(commands)?;
    let mut text = String::new();
    for (e, id) in last_commands.iter() {
        text.push_str(&format!("{}-{}: {:?}\n", e.id(), e.gen().id(), id));
    }
    commands.text = text;

    Some(())
}
