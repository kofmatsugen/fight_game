use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CollisionType {
    Extrusion,
    // 打撃
    Blow {
        damage: f32,      // ダメージ
        air: BlowInfo,    // 空中ヒット時
        ground: BlowInfo, // 地上ヒット時
        hit_level: HitLevel,
    },
    // 弾
    Projectile {
        damage: f32,      // ダメージ
        air: BlowInfo,    // 空中ヒット時
        ground: BlowInfo, // 地上ヒット時
        hit_level: HitLevel,
    },
    Throw,
    Damaged, // 被ダメージ
}

// 攻撃ヒット時の硬直情報
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BlowInfo {
    x: f32,       // 移動速度初期値
    y: f32,       // 移動速度初期値
    frame: usize, // ヒットフレーム
}

// ヒットレベル情報
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq)]
pub enum HitLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Custom { level: u8, frame: usize },
}

impl HitLevel {
    pub fn level(&self) -> u8 {
        use HitLevel::*;
        match self {
            Level1 => 1,
            Level2 => 2,
            Level3 => 3,
            Level4 => 4,
            &Custom { level, .. } => level,
        }
    }

    pub fn hitstop(&self) -> usize {
        use HitLevel::*;
        match self {
            Level1 => 12,
            Level2 => 15,
            Level3 => 18,
            Level4 => 24,
            &Custom { frame, .. } => frame,
        }
    }
}

impl Ord for HitLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level().cmp(&other.level())
    }
}

impl PartialOrd for HitLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HitLevel {
    fn eq(&self, other: &Self) -> bool {
        self.level().eq(&other.level())
    }
}
