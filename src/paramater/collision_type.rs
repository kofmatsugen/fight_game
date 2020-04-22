use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CollisionType {
    Extrusion,
    // 打撃
    Blow {
        damage: f32,      // ダメージ
        air: BlowInfo,    // 空中ヒット時
        ground: BlowInfo, // 地上ヒット時
    },
    // 弾
    Projectile {
        damage: f32,      // ダメージ
        air: BlowInfo,    // 空中ヒット時
        ground: BlowInfo, // 地上ヒット時
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
