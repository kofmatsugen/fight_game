use crate::input::{FightInput, InputFlag};
use amethyst::{
    ecs::{Entities, Entity, Read, ReadExpect, System, World, WriteStorage},
    shrev::{EventChannel, ReaderId},
    ui::{Anchor, LineMode, UiText, UiTransform},
    utils::circular_buffer::CircularBuffer,
};
use input_handle::traits::InputParser;

#[cfg(feature = "debug")]
use debug_system::DebugFont;

type InputEvent = <FightInput as InputParser>::Event;
type InputSignal = <FightInput as InputParser>::InputSignal;

pub struct InputDebugSystem {
    debug_ui_stick: Option<Entity>,
    debug_ui_button: Option<Entity>,
    debug_ui_event: Option<Entity>,
    _reader: ReaderId<InputEvent>,
    log_buffer: CircularBuffer<InputFlag>,
}

impl InputDebugSystem {
    pub fn new(world: &mut World) -> Self {
        InputDebugSystem {
            debug_ui_stick: None,
            debug_ui_button: None,
            debug_ui_event: None,
            _reader: world
                .fetch_mut::<EventChannel<InputEvent>>()
                .register_reader(),
            log_buffer: CircularBuffer::new(20),
        }
    }
}

impl<'s> System<'s> for InputDebugSystem {
    type SystemData = (
        ReadExpect<'s, DebugFont>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        Read<'s, InputSignal>,
        Read<'s, EventChannel<InputEvent>>,
        ReadExpect<'s, CircularBuffer<InputSignal>>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (debug_font, mut transforms, mut texts, signal, _event, input_buffer, entities) = data;

        // 実際の履歴の最後と履歴の最後が違ったら差分を記録
        let logged = match (
            input_buffer.queue().iter().last().map(|f| f.is_push_flag()),
            self.log_buffer.queue().iter().last(),
        ) {
            (Some(last), Some(&log_last)) => {
                let diff = last ^ log_last;
                if last.is_empty() || diff.is_empty() {
                    None
                } else {
                    Some(last)
                }
            }
            (Some(last), None) => Some(last),
            (_, _) => None,
        };

        if let Some(logged) = logged {
            self.log_buffer.push(logged);
        }

        update_event(
            &mut self.debug_ui_event,
            &mut transforms,
            &mut texts,
            &debug_font,
            &entities,
            &self.log_buffer,
        );
        update_stick_ui(
            &mut self.debug_ui_stick,
            &signal,
            &mut transforms,
            &mut texts,
            &debug_font,
            &entities,
        );
        update_stick_button(
            &mut self.debug_ui_button,
            &signal,
            &mut transforms,
            &mut texts,
            &debug_font,
            &entities,
        );
    }
}

fn update_stick_ui(
    debug_ui_stick: &mut Option<Entity>,
    signal: &InputSignal,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
) {
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

    if let Some(ui) = debug_ui_stick {
        let text = texts.get_mut(*ui).unwrap();
        text.text = input_text;
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
        *debug_ui_stick = Some(entity);
    }
}

fn update_stick_button(
    debug_ui_button: &mut Option<Entity>,
    signal: &InputSignal,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
) {
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

    if let Some(ui) = debug_ui_button {
        let text = texts.get_mut(*ui).unwrap();
        text.text = input_text;
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
        *debug_ui_button = Some(entity);
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

fn update_event(
    debug_ui_button: &mut Option<Entity>,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
    debug_font: &DebugFont,
    entities: &Entities,
    buffer: &CircularBuffer<InputFlag>,
) {
    let mut output = String::new();
    for f in buffer.queue().iter() {
        output.push_str(&format!("{}\n", f));
    }

    if let Some(ui) = debug_ui_button {
        let text = texts.get_mut(*ui).unwrap();
        text.text = output;
    } else {
        let system_font = debug_font.system_font.clone();
        let entity = entities.create();
        let transform = UiTransform::new(
            "fight_input_event".to_string(),
            Anchor::TopRight,
            Anchor::TopRight,
            0.,
            0.,
            0.,
            100.,
            800.,
        );
        let mut text = UiText::new(system_font, output, [0., 0., 0., 1.], 18.);
        text.line_mode = LineMode::Wrap;

        let _transform = transforms.insert(entity, transform);
        let _text = texts.insert(entity, text);
        *debug_ui_button = Some(entity);
    }
}
