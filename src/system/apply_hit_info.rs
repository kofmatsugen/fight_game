use crate::components::HitInfo;
use amethyst::core::ecs::{Join, System, WriteStorage};
use amethyst_sprite_studio::components::AnimationTime;

// ヒットストップの計算基準とするfps
const HIT_STOP_FPS: f32 = 60.;

// ヒット情報を適用する
pub struct ApplyHitInfoSystem;

impl ApplyHitInfoSystem {
    pub fn new() -> Self {
        ApplyHitInfoSystem
    }
}

impl<'s> System<'s> for ApplyHitInfoSystem {
    type SystemData = (WriteStorage<'s, HitInfo>, WriteStorage<'s, AnimationTime>);

    fn run(&mut self, (mut hits, mut times): Self::SystemData) {
        for (hit, time) in (&mut hits, &mut times).join() {
            // ヒット情報のリセット
            *hit = HitInfo::default();
        }
    }
}
