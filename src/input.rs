mod flag;
mod signal;

use crate::{
    binding::{Action, Axis, FightBindings},
    components::{Direction, PlayerTag},
    id::command::Command,
};
use amethyst::{
    ecs::{Entities, Entity, Join, ReadStorage},
    input::InputHandler,
    utils::circular_buffer::CircularBuffer,
};
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
    type SystemData = (Entities<'s>, ReadStorage<'s, Direction>);

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
        _buffer: &CircularBuffer<Self::InputSignal>,
        (entities, direction): Self::SystemData,
    ) -> Vec<Self::Event> {
        (&*entities, &direction)
            .join()
            .map(|(e, _direction)| (e, Command::A))
            .collect()
    }
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
