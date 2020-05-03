use amethyst::ecs::{Component, DenseVecStorage};

// ノックバックの残り時間
// ダメージ用のアニメーションはこのアニメーションが終わってからニュートラルに戻る
pub struct Knockback {
    rest_time: f32,
}

impl Knockback {
    pub(crate) fn new() -> Self {
        Knockback {
            rest_time: std::f32::MIN,
        }
    }

    pub(crate) fn set_knockback(&mut self, time: f32) {
        self.rest_time = time;
    }

    pub(crate) fn decrement(&mut self, time: f32) {
        self.rest_time -= time;
    }

    pub(crate) fn is_knockback(&self) -> bool {
        self.rest_time > 0.
    }
}

impl Component for Knockback {
    type Storage = DenseVecStorage<Self>;
}
