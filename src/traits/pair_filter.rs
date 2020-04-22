use crate::paramater::{CollisionParamater, CollisionType};
use amethyst::ecs::Entity;
use amethyst_aabb::traits::PairFilter;

impl<'s> PairFilter<'s> for CollisionParamater {
    type SystemData = ();

    // 判定処理を行うフィルタ
    fn pair_filter(
        _entity1: Entity,
        p1: &Self,
        _entity2: Entity,
        p2: &Self,
        _data: &Self::SystemData,
    ) -> bool {
        match (p1.collision_type, p2.collision_type) {
            (CollisionType::Extrusion, CollisionType::Extrusion) => true,
            (CollisionType::Damaged, CollisionType::Blow { .. }) => true,
            (CollisionType::Damaged, CollisionType::Projectile { .. }) => true,
            (CollisionType::Damaged, CollisionType::Throw) => true,
            (CollisionType::Blow { .. }, CollisionType::Damaged) => true,
            (CollisionType::Projectile { .. }, CollisionType::Damaged) => true,
            (CollisionType::Throw, CollisionType::Damaged) => true,
            _ => false,
        }
    }
}
