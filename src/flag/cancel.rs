use crate::id::command::Command;
use serde::{
    de::{SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

bitflags::bitflags! {
    // キャンセル可能フラグ
    #[derive(Default)]
    pub struct Cancel : u64 {
        const SPECIAL_SKILL = 1 << 0;   // 必殺技(A -> B -> C -> D)
        const FRONT_JUMP = 1 << 1;    // 前ジャンプ
        const VERTICAL_JUMP = 1 << 2;    // 垂直ジャンプ
        const BACK_JUMP = 1 << 3;    // 後ろジャンプ
        const BARRAGE = 1 << 4; // 連打
        const NORMAL_SKILL = 1 << 5; // 通常技(通常技)
        const WALK = 1 << 6;    // 前移動
        const BACK = 1 << 7;    // 後ろ移動
        const FRONT_DASH = 1 << 8;    // ダッシュ
        const BACK_DASH = 1 << 9;    // 後ろダッシュ
        const CROUCH = 1 << 10; // しゃがみ
        const STANCE = 1 << 11; // ニュートラル
        const CROUCH_GUARD = 1 << 12; // しゃがみガード
        const STAND_GUARD = 1 << 13;    // 立ちガード

        // 設定用エイリアス
        const MOVE = Self::WALK.bits | Self::BACK.bits;   // 通常移動
        const DASH = Self::FRONT_DASH.bits | Self::BACK_DASH.bits;   // ダッシュ移動
        const JUMP = Self::FRONT_JUMP.bits | Self::VERTICAL_JUMP.bits | Self::BACK_JUMP.bits;   // ジャンプ移動
        const SKILL = Self::SPECIAL_SKILL.bits | Self::NORMAL_SKILL.bits;   // 技全般
        const GUARD = Self::CROUCH_GUARD.bits | Self::STAND_GUARD.bits; // ガード
    }
}

// 指定のコマンドでキャンセル可能か判定
impl Cancel {
    pub fn is_cancelable(&self, command: &Command) -> bool {
        match command {
            Command::Back => self.contains(Cancel::BACK),
            Command::Walk => self.contains(Cancel::WALK),
            Command::BackDash => self.contains(Cancel::BACK_DASH),
            Command::Dash => self.contains(Cancel::FRONT_DASH),
            Command::VerticalJump => self.contains(Cancel::VERTICAL_JUMP),
            Command::BackJump => self.contains(Cancel::BACK_JUMP),
            Command::FrontJump => self.contains(Cancel::FRONT_JUMP),
            Command::Crouch => self.contains(Cancel::CROUCH),
            Command::BackCrouch => self.contains(Cancel::CROUCH),
            Command::FrontCrouch => self.contains(Cancel::CROUCH),
            Command::A => self.contains(Cancel::NORMAL_SKILL),
            Command::B => self.contains(Cancel::NORMAL_SKILL),
            Command::C => self.contains(Cancel::NORMAL_SKILL),
            Command::D => self.contains(Cancel::NORMAL_SKILL),
        }
    }
}

impl<'de> Deserialize<'de> for Cancel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CancelVisitor)
    }
}

struct CancelVisitor;

// キャンセルフラグをデシリアライズする用のenum
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum CancelValue {
    Special,
    Normal,
    FrontJump,
    VerticalJump,
    BackJump,
    Barrage,
    Walk,
    Back,
    FrontDash,
    BackDash,
    Crouch,
    Stance,
    StandGuard,
    CrouchGuard,

    // 設定用エイリアス
    Move,
    Dash,
    Jump,
    Skill,
    Guard,
}

// シリアライズ用フラグ優先順位
const SERIALIZE_FLAGS: [(Cancel, CancelValue); 19] = [
    // 設定用エイリアス
    (Cancel::MOVE, CancelValue::Move),
    (Cancel::DASH, CancelValue::Dash),
    (Cancel::JUMP, CancelValue::Jump),
    (Cancel::SKILL, CancelValue::Skill),
    (Cancel::GUARD, CancelValue::Guard),
    // 通常フラグ
    (Cancel::SPECIAL_SKILL, CancelValue::Special),
    (Cancel::NORMAL_SKILL, CancelValue::Normal),
    (Cancel::FRONT_JUMP, CancelValue::FrontJump),
    (Cancel::VERTICAL_JUMP, CancelValue::VerticalJump),
    (Cancel::BACK_JUMP, CancelValue::BackJump),
    (Cancel::BARRAGE, CancelValue::Barrage),
    (Cancel::WALK, CancelValue::Walk),
    (Cancel::BACK, CancelValue::Back),
    (Cancel::FRONT_DASH, CancelValue::FrontDash),
    (Cancel::BACK_DASH, CancelValue::BackDash),
    (Cancel::CROUCH, CancelValue::Crouch),
    (Cancel::STANCE, CancelValue::Stance),
    (Cancel::STAND_GUARD, CancelValue::StandGuard),
    (Cancel::CROUCH_GUARD, CancelValue::CrouchGuard),
];

impl CancelValue {
    fn convert_flag(self) -> Cancel {
        match self {
            CancelValue::Special => Cancel::SPECIAL_SKILL,
            CancelValue::Normal => Cancel::NORMAL_SKILL,
            CancelValue::FrontJump => Cancel::FRONT_JUMP,
            CancelValue::VerticalJump => Cancel::VERTICAL_JUMP,
            CancelValue::BackJump => Cancel::BACK_JUMP,
            CancelValue::Barrage => Cancel::BARRAGE,
            CancelValue::Walk => Cancel::WALK,
            CancelValue::Back => Cancel::BACK,
            CancelValue::FrontDash => Cancel::FRONT_DASH,
            CancelValue::BackDash => Cancel::BACK_DASH,
            CancelValue::Crouch => Cancel::CROUCH,
            CancelValue::Stance => Cancel::STANCE,
            CancelValue::StandGuard => Cancel::STAND_GUARD,
            CancelValue::CrouchGuard => Cancel::CROUCH_GUARD,

            // 設定用エイリアス
            CancelValue::Move => Cancel::MOVE,
            CancelValue::Dash => Cancel::DASH,
            CancelValue::Jump => Cancel::JUMP,
            CancelValue::Skill => Cancel::SKILL,
            CancelValue::Guard => Cancel::GUARD,
        }
    }

    fn from_flag(mut flag: Cancel) -> Vec<Self> {
        SERIALIZE_FLAGS
            .iter()
            .filter_map(|&(f, v)| {
                let val = if flag.contains(f) == true {
                    Some(v)
                } else {
                    None
                };

                flag.remove(f);

                val
            })
            .collect()
    }
}

impl<'de> Visitor<'de> for CancelVisitor {
    type Value = Cancel;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("not supported format")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut cancel = Cancel::empty();
        while let Some(elem) = seq.next_element()? {
            let flag = CancelValue::convert_flag(elem);
            cancel |= flag;
        }
        Ok(cancel)
    }
}

impl Serialize for Cancel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let values = CancelValue::from_flag(*self);
        let mut seq = serializer.serialize_seq(Some(values.len()))?;
        for v in values {
            seq.serialize_element(&v)?;
        }
        seq.end()
    }
}
