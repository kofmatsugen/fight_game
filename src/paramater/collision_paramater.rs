use amethyst::core::Transform;
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_collision::traits::ContactEventParamater;

#[derive(Debug)]
pub struct CollisionParamater;

impl<'a> ContactEventParamater<'a> for CollisionParamater {
    type CollisionParamater = (); // 判定そのものに付いているパラメータ
    type OptionParamater = (ReadStorage<'a, Transform>,); // 外部から渡されるパラメータ

    // 判定のパラメータとオプションのパラメータからイベントパラメータを生成
    fn create_paramater(
        _e: Entity,
        (): &Self::CollisionParamater,
        (_transform,): &Self::OptionParamater,
    ) -> Self {
        CollisionParamater
    }
}
