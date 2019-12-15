use amethyst::core::Transform;
use amethyst_collision::traits::ContactEventParamater;

#[derive(Debug)]
pub struct CollisionParamater;

impl ContactEventParamater for CollisionParamater {
    type CollisionParamater = (); // 判定そのものに付いているパラメータ
    type OptionParamater = Transform; // 外部から渡されるパラメータ

    // 判定のパラメータとオプションのパラメータからイベントパラメータを生成
    fn create_paramater(_: &Self::CollisionParamater, _: Option<&Self::OptionParamater>) -> Self {
        CollisionParamater
    }
}
