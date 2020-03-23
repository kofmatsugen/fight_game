use crate::paramater::CollisionParamater;
use amethyst::ecs::Entity;
use amethyst_collision::traits::ContactEventParamater;

#[derive(Debug)]
pub struct ContactParamter;

impl<'a> ContactEventParamater<'a> for ContactParamter {
    type BaseParamater = CollisionParamater; // 衝突イベントパラメータ生成用のパラメータ
    type OptionParamater = (); // 外部から渡されるパラメータ

    // 判定のパラメータとオプションのパラメータからイベントパラメータを生成
    fn create_paramater(
        _e: Entity,
        _param: &Self::BaseParamater,
        (): &Self::OptionParamater,
    ) -> Self {
        ContactParamter
    }
}
