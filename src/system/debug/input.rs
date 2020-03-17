use crate::{
    components::PlayerTag,
    input::{FightInput, InputFlag, InputSignal},
};
use amethyst::{
    ecs::{Entity, ReadExpect, System, World, WriteStorage},
    ui::{UiFinder, UiText},
    utils::circular_buffer::CircularBuffer,
};
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
        UiFinder<'s>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CircularBuffer<<FightInput as InputParser<'s>>::InputSignal>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (finder, mut texts, input_buffer) = data;

        self.update_log(PlayerTag::P1, &input_buffer);
        self.update_log(PlayerTag::P2, &input_buffer);

        self.find_ui(&finder, PlayerTag::P1);
        self.find_ui(&finder, PlayerTag::P2);

        self.update_ui(PlayerTag::P1, &mut texts);
        self.update_ui(PlayerTag::P2, &mut texts);
    }
}

impl InputDebugSystem {
    fn find_ui(&mut self, finder: &UiFinder, tag: PlayerTag) {
        // 各種UIを探してくる
        if self.debug_ui_event.contains_key(&tag) == false {
            finder.find(&format!("debug_event_{:?}", tag)).map(|ui| {
                self.debug_ui_event.insert(tag, ui);
            });
        }
        if self.debug_ui_button.contains_key(&tag) == false {
            finder.find(&format!("debug_button_{:?}", tag)).map(|ui| {
                self.debug_ui_button.insert(tag, ui);
            });
        }
        if self.debug_ui_stick.contains_key(&tag) == false {
            finder.find(&format!("debug_stick_{:?}", tag)).map(|ui| {
                self.debug_ui_stick.insert(tag, ui);
            });
        }
    }

    fn update_log<'s>(
        &mut self,
        tag: PlayerTag,
        input_buffer: &CircularBuffer<<FightInput as InputParser<'s>>::InputSignal>,
    ) -> Option<()> {
        let input_last = input_buffer
            .queue()
            .iter()
            .last()
            .and_then(|f| f.get(&tag))?;

        // 実際の履歴の最後と履歴の最後が違ったら差分を記録
        let logged = match self.last_key.get(&tag) {
            Some(log_last) => {
                let diff = input_last.is_push_flag() ^ log_last.is_push_flag();
                if diff.is_empty() {
                    None
                } else {
                    Some(input_last)
                }
            }
            _ => None,
        };

        let entry = self
            .log_buffer
            .entry(tag)
            .or_insert(CircularBuffer::new(BUFFER_SIZE));

        if let Some(logged) = logged {
            if logged.is_push_flag().is_empty() == false {
                entry.push(logged.is_push_flag());
            }
        }

        self.last_key.insert(tag, *input_last);

        Some(())
    }

    fn update_ui<'s>(
        &mut self,
        tag: PlayerTag,
        texts: &mut WriteStorage<'s, UiText>,
    ) -> Option<()> {
        update_event(
            self.debug_ui_event.get(&tag),
            texts,
            self.log_buffer.get(&tag)?, // 上で絶対入れるので[]アクセスでOK
        );

        update_stick_ui(
            self.debug_ui_stick.get(&tag),
            &self.last_key[&tag], // 上で絶対入れるので[]アクセスでOK
            texts,
        );

        update_stick_button(
            self.debug_ui_button.get(&tag),
            &self.last_key[&tag], // 上で絶対入れるので[]アクセスでOK
            texts,
        );
        Some(())
    }
}

fn update_stick_ui(
    debug_ui_stick: Option<&Entity>,
    signal: &InputSignal,
    texts: &mut WriteStorage<UiText>,
) -> Option<()> {
    let ui = debug_ui_stick?;

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

    let text = texts.get_mut(*ui)?;
    text.text = input_text;

    Some(())
}

fn update_stick_button(
    debug_ui_button: Option<&Entity>,
    signal: &InputSignal,
    texts: &mut WriteStorage<UiText>,
) -> Option<()> {
    let ui = debug_ui_button?;

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

    let text = texts.get_mut(*ui)?;
    text.text = input_text;

    Some(())
}

fn update_event(
    debug_ui_button: Option<&Entity>,
    texts: &mut WriteStorage<UiText>,
    buffer: &CircularBuffer<InputFlag>,
) -> Option<()> {
    let ui = debug_ui_button?;
    let mut output = String::new();
    for f in buffer.queue().iter() {
        output.push_str(&format!("{}\n", f));
    }

    let text_ui = texts.get_mut(*ui)?;
    text_ui.text = output;

    Some(())
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
