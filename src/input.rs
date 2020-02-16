use crate::binding::{Action, Axis, FightBindings, PlayerTag};
use amethyst::{input::InputHandler, utils::circular_buffer::CircularBuffer};
use input_handle::traits::InputParser;

pub struct FightInput;

const AXIS_THRESHOLD: f32 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct InputSignal {
    is_down: InputFlag,
    is_push: InputFlag,
    is_release: InputFlag,
}

impl InputSignal {
    pub fn is_down_flag(&self) -> InputFlag {
        self.is_down
    }
    pub fn is_down(&self, flag: InputFlag) -> bool {
        self.is_down.contains(flag)
    }
    pub fn is_push_flag(&self) -> InputFlag {
        self.is_push
    }
    pub fn is_push(&self, flag: InputFlag) -> bool {
        self.is_push.contains(flag)
    }
    pub fn is_release_flag(&self) -> InputFlag {
        self.is_release
    }
    pub fn is_release(&self, flag: InputFlag) -> bool {
        self.is_release.contains(flag)
    }
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct InputFlag : u64{
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;

        const DOWN = 1 << 4;
        const UP = 1 << 5;
        const RIGHT = 1 << 6;
        const LEFT = 1 << 7;

        const RIGHT_DOWN = 1 << 8;
        const LEFT_DOWN = 1 << 9;
        const RIGHT_UP = 1 << 10;
        const LEFT_UP = 1 << 11;
    }
}

impl std::fmt::Display for InputFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.contains(Self::UP) {
            write!(f, "8")?;
        }
        if self.contains(Self::DOWN) {
            write!(f, "2")?;
        }
        if self.contains(Self::RIGHT) {
            write!(f, "6")?;
        }
        if self.contains(Self::LEFT) {
            write!(f, "4")?;
        }
        if self.contains(Self::RIGHT_UP) {
            write!(f, "9")?;
        }
        if self.contains(Self::RIGHT_DOWN) {
            write!(f, "3")?;
        }
        if self.contains(Self::LEFT_UP) {
            write!(f, "7")?;
        }
        if self.contains(Self::LEFT_DOWN) {
            write!(f, "1")?;
        }
        if self.contains(Self::A) {
            write!(f, "A")?;
        }
        if self.contains(Self::B) {
            write!(f, "B")?;
        }
        if self.contains(Self::C) {
            write!(f, "C")?;
        }
        if self.contains(Self::D) {
            write!(f, "D")?;
        }
        write!(f, "")
    }
}

impl InputParser for FightInput {
    const BUFFER_SIZE: usize = 120; // 入力を覚えるF数
    type BindingTypes = FightBindings; // 入力キー
    type InputSignal = InputSignal; // バッファに詰める入力(ビットフラグなどでまとめる)
    type Event = (); // バッファに詰めた入力から生成された実際に各エンティティにくばるイベント
    type SystemData = ();

    // 入力を確認してバッファに信号を生成する
    fn add_buffer(
        handler: &InputHandler<Self::BindingTypes>,
        prev_input: Option<Self::InputSignal>,
    ) -> Self::InputSignal {
        let mut signal = InputSignal {
            is_down: InputFlag::empty(),
            is_push: InputFlag::empty(),
            is_release: InputFlag::empty(),
        };

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

    // バッファの信号をパースして処理するためのイベントを生成する
    // 格ゲーのコマンド入力とかに使う
    fn parse_input(
        _buffer: &CircularBuffer<Self::InputSignal>,
        _system: &mut Self::SystemData,
    ) -> Option<Self::Event> {
        None
    }
}
