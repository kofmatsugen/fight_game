use crate::resource::command::{CommandList, CommandStore};
use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Read, ReadExpect, System, World, WriteStorage},
    ui::{Anchor, LineMode, UiText, UiTransform},
};
use debug_system::DebugFont;

pub struct CommandDebugSystem {
    debug_commands: Option<Entity>,
}

impl CommandDebugSystem {
    pub fn new(_world: &mut World) -> Self {
        CommandDebugSystem {
            debug_commands: None,
        }
    }
}

impl<'s> System<'s> for CommandDebugSystem {
    type SystemData = (
        ReadExpect<'s, DebugFont>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        Entities<'s>,
        ReadExpect<'s, CommandStore>,
        Read<'s, AssetStorage<CommandList>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (debug_font, mut transforms, mut texts, entities, store, list) = data;
        update_loaded(
            &mut self.debug_commands,
            &mut transforms,
            &mut texts,
            &debug_font,
            &entities,
            &store,
            &list,
        );
    }
}

fn update_loaded(
    command_list: &mut Option<Entity>,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
    store: &CommandStore,
    list: &AssetStorage<CommandList>,
) {
    let mut input_text = String::new();
    for (id, handle) in store.commands() {
        if let Some(list) = list.get(handle) {
            input_text.push_str(&format!("{}: \n", id));
            for (id, c) in list.commands() {
                input_text.push_str(&format!("\t{:?}: {}\n", id, c));
            }
        }
    }

    if let Some(ui) = command_list {
        let text = texts.get_mut(*ui).unwrap();
        text.text = input_text;
    } else {
        let system_font = debug_font.system_font.clone();
        let entity = entities.create();
        let transform = UiTransform::new(
            "command_list".to_string(),
            Anchor::MiddleLeft,
            Anchor::TopLeft,
            0.,
            0.,
            0.,
            200.,
            500.,
        );
        let mut text = UiText::new(system_font, input_text, [0., 0., 0., 1.], 16.);
        text.line_mode = LineMode::Wrap;

        let _transform = transforms.insert(entity, transform);
        let _text = texts.insert(entity, text);
        *command_list = Some(entity);
    }
}
