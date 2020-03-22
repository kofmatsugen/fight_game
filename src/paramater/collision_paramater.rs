use crate::paramater::AnimationParam;
use amethyst::core::Transform;
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_collision::traits::ContactEventParamater;

#[derive(Debug)]
pub struct CollisionParamater;

impl<'a> ContactEventParamater<'a> for CollisionParamater {
    type CollisionParamater = AnimationParam; // 衝突判定時のイベントを生成するためのパラメータ
    type OptionParamater = (ReadStorage<'a, Transform>,); // 外部から渡されるパラメータ

    // 判定のパラメータとオプションのパラメータからイベントパラメータを生成
    fn create_paramater(
        _e: Entity,
        _param: &Self::CollisionParamater,
        (_transform,): &Self::OptionParamater,
    ) -> Self {
        CollisionParamater
    }
}
