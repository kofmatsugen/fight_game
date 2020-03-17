mod flag;
mod signal;

use crate::{
    binding::{Action, Axis, FightBindings},
    components::{Direction, PlayerTag},
    id::command::Command,
    resource::command::{CommandList, CommandStore},
};
use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage},
    input::InputHandler,
    utils::circular_buffer::CircularBuffer,
};
use command_parser::Key;
use input_handle::traits::InputParser;
use std::collections::BTreeMap;

pub use flag::InputFlag;
pub use signal::InputSignal;

pub struct FightInput;

const AXIS_THRESHOLD: f32 = 0.2;

impl<'s> InputParser<'s> for FightInput {
    const BUFFER_SIZE: usize = 120; // 入力を覚えるF数
    type BindingTypes = FightBindings; // 入力キー
    type InputSignal = BTreeMap<PlayerTag, InputSignal>; // バッファに詰める入力(ビットフラグなどでまとめる)
    type Event = (Entity, Command); // バッファに詰めた入力から生成された実際に各エンティティにくばるイベント
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, Direction>,
        ReadExpect<'s, CommandStore>,
        Read<'s, AssetStorage<CommandList>>,
    );

    // 入力を確認してバッファに信号を生成する
    fn add_buffer(
        handler: &InputHandler<Self::BindingTypes>,
        prev_input: Option<&Self::InputSignal>,
    ) -> Self::InputSignal {
        let mut signal = BTreeMap::default();

        let player1 = make_signal(handler, prev_input.and_then(|s| s.get(&PlayerTag::P1)));
        let player2 = make_signal(handler, prev_input.and_then(|s| s.get(&PlayerTag::P2)));

        signal.insert(PlayerTag::P1, player1);
        signal.insert(PlayerTag::P2, player2);

        signal
    }

    // バッファの信号をパースして処理するためのイベントを生成する
    // 格ゲーのコマンド入力とかに使う
    // コマンドの左右判定はKOFと同等(現フレームの向きでパース)
    // SF系(入力時に向きでパース)する場合は add_buffer で処理
    fn parse_input(
        buffer: &CircularBuffer<Self::InputSignal>,
        (entities, tags, direction, store, storage): Self::SystemData,
    ) -> Vec<Self::Event> {
        (&*entities, &tags, &direction)
            .join()
            .filter_map(|(e, tag, direction)| {
                store
                    .commands()
                    .filter_map(|handle| storage.get(handle).map(|l| l.commands()))
                    .flatten()
                    .find_map(|(id, commands)| {
                        let judge_ok = commands.iter().any(|command| {
                            command.judge_inputs(
                                buffer.queue().iter().filter_map(|signal| {
                                    let signal = signal.get(tag)?;
                                    Some(convert_command_input(signal, direction))
                                }),
                                8,
                                8,
                            )
                        });
                        if judge_ok {
                            Some((e, *id))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

// 各フラグをコマンド判定用の値に変換
fn convert_command_input(signal: &InputSignal, direction: &Direction) -> Key {
    let mut key = Key::empty();
    let down = signal.is_down_flag();

    if down.contains(InputFlag::A) {
        key |= Key::A;
    }
    if down.contains(InputFlag::B) {
        key |= Key::B;
    }
    if down.contains(InputFlag::C) {
        key |= Key::C;
    }
    if down.contains(InputFlag::D) {
        key |= Key::D;
    }

    // 左右は向きによって変わる
    let mut is_neutral = true;
    if down.contains(InputFlag::DOWN) {
        key |= Key::DOWN;
        is_neutral = false;
    }
    if down.contains(InputFlag::UP) {
        key |= Key::UP;
        is_neutral = false;
    }
    if down.contains(InputFlag::RIGHT) {
        match direction {
            Direction::Right => key |= Key::FORWARD,
            Direction::Left => key |= Key::BACKWARD,
        }
        is_neutral = false;
    }
    if down.contains(InputFlag::LEFT) {
        match direction {
            Direction::Right => key |= Key::BACKWARD,
            Direction::Left => key |= Key::FORWARD,
        }
        is_neutral = false;
    }

    if down.contains(InputFlag::RIGHT_DOWN) {
        match direction {
            Direction::Right => key |= Key::FD,
            Direction::Left => key |= Key::BD,
        }
        is_neutral = false;
    }
    if down.contains(InputFlag::LEFT_DOWN) {
        match direction {
            Direction::Right => key |= Key::BD,
            Direction::Left => key |= Key::FD,
        }
        is_neutral = false;
    }
    if down.contains(InputFlag::RIGHT_UP) {
        match direction {
            Direction::Right => key |= Key::FU,
            Direction::Left => key |= Key::BU,
        }
        is_neutral = false;
    }
    if down.contains(InputFlag::LEFT_UP) {
        match direction {
            Direction::Right => key |= Key::BU,
            Direction::Left => key |= Key::FU,
        }
        is_neutral = false;
    }

    // どの方向も入ってなければニュートラル
    if is_neutral {
        key |= Key::NEUTRAL;
    }

    key
}

fn make_signal(
    handler: &InputHandler<<FightInput as InputParser>::BindingTypes>,
    prev_input: Option<&InputSignal>,
) -> InputSignal {
    let mut signal = InputSignal::default();

    if let Some(true) = handler.action_is_down(&Action::A(PlayerTag::P1)) {
        signal.is_down |= InputFlag::A;
    }
    if let Some(true) = handler.action_is_down(&Action::B(PlayerTag::P1)) {
        signal.is_down |= InputFlag::B;
    }
    if let Some(true) = handler.action_is_down(&Action::C(PlayerTag::P1)) {
        signal.is_down |= InputFlag::C;
    }
    if let Some(true) = handler.action_is_down(&Action::D(PlayerTag::P1)) {
        signal.is_down |= InputFlag::D;
    }

    match (
        handler.axis_value(&Axis::Right(PlayerTag::P1)),
        handler.axis_value(&Axis::Up(PlayerTag::P1)),
    ) {
        (Some(lr), Some(ud)) => {
            if ud > AXIS_THRESHOLD && lr > AXIS_THRESHOLD {
                signal.is_down |= InputFlag::RIGHT_UP;
            } else if ud > AXIS_THRESHOLD && lr < -AXIS_THRESHOLD {
                signal.is_down |= InputFlag::LEFT_UP;
            } else if ud < -AXIS_THRESHOLD && lr > AXIS_THRESHOLD {
                signal.is_down |= InputFlag::RIGHT_DOWN;
            } else if ud < -AXIS_THRESHOLD && lr < -AXIS_THRESHOLD {
                signal.is_down |= InputFlag::LEFT_DOWN;
            } else if lr > AXIS_THRESHOLD {
                signal.is_down |= InputFlag::RIGHT;
            } else if lr < -AXIS_THRESHOLD {
                signal.is_down |= InputFlag::LEFT;
            } else if ud > AXIS_THRESHOLD {
                signal.is_down |= InputFlag::UP;
            } else if ud < -AXIS_THRESHOLD {
                signal.is_down |= InputFlag::DOWN;
            }
        }
        _ => {}
    }

    if let Some(prev) = prev_input {
        signal.is_push = signal.is_down & (signal.is_down ^ prev.is_down);
        signal.is_release = !signal.is_down & (signal.is_down ^ prev.is_down);
    } else {
        signal.is_push = signal.is_down;
    }

    if signal.is_push.is_empty() == false {
        log::info!("push: {:?}", signal.is_push);
    }
    if signal.is_release.is_empty() == false {
        log::info!("release: {:?}", signal.is_release);
    }

    signal
}
