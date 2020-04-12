use crate::paramater::{CollisionParamater, CollisionType};
use amethyst::ecs::Entity;
use amethyst_collision::traits::ContactEventParamater;

#[derive(Debug)]
pub struct ContactParamter {
    pub collision_type: CollisionType,
}

impl<'a> ContactEventParamater<'a> for ContactParamter {
    type BaseParamater = CollisionParamater; // 衝突イベントパラメータ生成用のパラメータ
    type OptionParamater = (); // 外部から渡されるパラメータ

    // 判定のパラメータとオプションのパラメータからイベントパラメータを生成
    fn create_paramater(
        _e: Entity,
        &CollisionParamater { collision_type, .. }: &Self::BaseParamater,
        (): &Self::OptionParamater,
    ) -> Self {
        ContactParamter { collision_type }
    }
}
