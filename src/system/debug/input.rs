use crate::{
    components::PlayerTag,
    input::{FightInput, InputFlag, InputSignal},
};
use amethyst::{
    ecs::{Entities, Entity, ReadExpect, System, World, WriteStorage},
    ui::{Anchor, LineMode, UiText, UiTransform},
    utils::circular_buffer::CircularBuffer,
};
use debug_system::DebugFont;
use input_handle::traits::InputParser;
use std::collections::BTreeMap;

pub struct InputDebugSystem {
    debug_ui_stick: BTreeMap<PlayerTag, Entity>,
    debug_ui_button: BTreeMap<PlayerTag, Entity>,
    debug_ui_event: BTreeMap<PlayerTag, Entity>,
    log_buffer: BTreeMap<PlayerTag, CircularBuffer<InputFlag>>,
    last_key: BTreeMap<PlayerTag, InputSignal>,
}

const BUFFER_SIZE: usize = 20;

impl InputDebugSystem {
    pub fn new(_: &mut World) -> Self {
        InputDebugSystem {
            debug_ui_stick: Default::default(),
            debug_ui_button: Default::default(),
            debug_ui_event: Default::default(),
            log_buffer: Default::default(),
            last_key: Default::default(),
        }
    }
}

impl<'s> System<'s> for InputDebugSystem {
    type SystemData = (
        ReadExpect<'s, DebugFont>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CircularBuffer<<FightInput as InputParser<'s>>::InputSignal>>,
        Entities<'s>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        self.update_ui(&mut data, PlayerTag::P1);
        //self.update_ui(&mut data, PlayerTag::P2);
    }
}

impl InputDebugSystem {
    fn update_ui<'s>(&mut self, data: &mut <Self as System<'s>>::SystemData, tag: PlayerTag) {
        let (debug_font, transforms, texts, input_buffer, entities) = data;
        // 実際の履歴の最後と履歴の最後が違ったら差分を記録
        let logged = match (
            input_buffer
                .queue()
                .iter()
                .last()
                .and_then(|f| f.get(&tag))
                .map(|f| f.is_push_flag()),
            self.last_key.get(&tag),
        ) {
            (Some(last), Some(log_last)) => {
                let diff = last ^ log_last.is_push_flag();
                if diff.is_empty() {
                    None
                } else {
                    Some(last)
                }
            }
            (_, _) => None,
        };

        match logged {
            Some(logged) => {
                if logged.is_empty() == false {
                    self.log_buffer
                        .entry(tag)
                        .or_insert(CircularBuffer::new(BUFFER_SIZE))
                        .push(logged);
                }
                if let Some(signal) = input_buffer.queue().iter().last().and_then(|f| f.get(&tag)) {
                    self.last_key.insert(tag, *signal);
                }
            }
            None => {
                self.log_buffer
                    .entry(tag)
                    .or_insert(CircularBuffer::new(BUFFER_SIZE));
                self.last_key.entry(tag).or_insert(InputSignal::default());
            }
        }

        let event_ui = update_event(
            self.debug_ui_event.get(&tag),
            transforms,
            texts,
            &debug_font,
            &entities,
            &self.log_buffer[&tag], // 上で絶対入れるので[]アクセスでOK
        );
        self.debug_ui_event.entry(tag).or_insert(event_ui);

        let stick_ui = update_stick_ui(
            self.debug_ui_stick.get(&tag),
            &self.last_key[&tag], // 上で絶対入れるので[]アクセスでOK
            transforms,
            texts,
            &debug_font,
            &entities,
        );
        self.debug_ui_stick.entry(tag).or_insert(stick_ui);

        let button_ui = update_stick_button(
            self.debug_ui_button.get(&tag),
            &self.last_key[&tag], // 上で絶対入れるので[]アクセスでOK
            transforms,
            texts,
            &debug_font,
            &entities,
        );
        self.debug_ui_button.entry(tag).or_insert(button_ui);
    }
}

fn update_stick_ui(
    debug_ui_stick: Option<&Entity>,
    signal: &InputSignal,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
) -> Entity {
    let up = format!(
        "{} {} {}",
        ox_(&signal, InputFlag::LEFT_UP),
        ox_(&signal, InputFlag::UP),
        ox_(&signal, InputFlag::RIGHT_UP),
    );
    let center = format!(
        "{} {} {}",
        ox_(&signal, InputFlag::LEFT),
        " ",
        ox_(&signal, InputFlag::RIGHT),
    );
    let down = format!(
        "{} {} {}",
        ox_(&signal, InputFlag::LEFT_DOWN),
        ox_(&signal, InputFlag::DOWN),
        ox_(&signal, InputFlag::RIGHT_DOWN),
    );

    let input_text = format!("{}\n{}\n{}", up, center, down);

    if let Some(&ui) = debug_ui_stick {
        let text = texts.get_mut(ui).unwrap();
        text.text = input_text;
        ui
    } else {
        let system_font = debug_font.system_font.clone();
        let entity = entities.create();
        let transform = UiTransform::new(
            "fight_input_stick".to_string(),
            Anchor::BottomRight,
            Anchor::BottomLeft,
            -200.,
            0.,
            0.,
            200.,
            200.,
        );
        let mut text = UiText::new(system_font, input_text, [0., 0., 0., 1.], 18.);
        text.line_mode = LineMode::Wrap;

        let _transform = transforms.insert(entity, transform);
        let _text = texts.insert(entity, text);
        entity
    }
}

fn update_stick_button(
    debug_ui_button: Option<&Entity>,
    signal: &InputSignal,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
) -> Entity {
    let up = format!(
        "{} {}",
        ox_(&signal, InputFlag::A),
        ox_(&signal, InputFlag::C),
    );
    let down = format!(
        "{} {}",
        ox_(&signal, InputFlag::B),
        ox_(&signal, InputFlag::D),
    );

    let input_text = format!("{}\n{}", up, down);

    if let Some(&ui) = debug_ui_button {
        let text = texts.get_mut(ui).unwrap();
        text.text = input_text;
        ui
    } else {
        let system_font = debug_font.system_font.clone();
        let entity = entities.create();
        let transform = UiTransform::new(
            "fight_input_button".to_string(),
            Anchor::BottomRight,
            Anchor::BottomLeft,
            -100.,
            0.,
            0.,
            100.,
            200.,
        );
        let mut text = UiText::new(system_font, input_text, [0., 0., 0., 1.], 18.);
        text.line_mode = LineMode::Wrap;

        let _transform = transforms.insert(entity, transform);
        let _text = texts.insert(entity, text);
        entity
    }
}

fn update_event(
    debug_ui_button: Option<&Entity>,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
    buffer: &CircularBuffer<InputFlag>,
) -> Entity {
    let mut output = String::new();
    for f in buffer.queue().iter() {
        output.push_str(&format!("{}\n", f));
    }

    if let Some(&ui) = debug_ui_button {
        let text = texts.get_mut(ui).unwrap();
        text.text = output;
        ui
    } else {
        let system_font = debug_font.system_font.clone();
        let entity = entities.create();
        let transform = UiTransform::new(
            "fight_input_event".to_string(),
            Anchor::TopRight,
            Anchor::TopRight,
            -30.,
            -30.,
            0.,
            140.,
            400.,
        );
        let mut text = UiText::new(system_font, output, [0., 0., 0., 1.], 18.);
        text.line_mode = LineMode::Wrap;
        text.align = Anchor::TopRight;

        let _transform = transforms.insert(entity, transform);
        let _text = texts.insert(entity, text);
        entity
    }
}

fn ox_(signal: &InputSignal, flag: InputFlag) -> &'static str {
    if signal.is_push(flag) {
        "x"
    } else if signal.is_down(flag) {
        "o"
    } else {
        "-"
    }
}
