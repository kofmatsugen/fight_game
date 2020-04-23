use crate::{
    paramater::{AnimationParam, CollisionType},
    traits::{ExtrudeFilter, ParamaterFromData},
};
use amethyst::ecs::Entity;
#[cfg(feature = "debug")]
use amethyst_aabb::debug::traits::CollisionColor;

#[derive(Clone, Copy, Debug)]
pub struct CollisionParamater {
    pub collision_type: CollisionType,
}

impl ParamaterFromData<AnimationParam> for CollisionParamater {
    fn make_collision_data(param: Option<&AnimationParam>) -> Option<Self> {
        let collision_type = param?.collision_type?;

        Some(CollisionParamater { collision_type })
    }
}

impl<'s> ExtrudeFilter<'s> for CollisionParamater {
    type SystemData = ();

    // 押し出し判定を行うフィルタ
    fn extrude_filter(
        _entity1: Entity,
        p1: &Self,
        _entity2: Entity,
        p2: &Self,
        _data: &Self::SystemData,
    ) -> bool {
        match (p1.collision_type, p2.collision_type) {
            (CollisionType::Extrusion, CollisionType::Extrusion) => true,
            _ => false,
        }
    }
}

#[cfg(feature = "debug")]
impl CollisionColor for CollisionParamater {
    fn collision_color(&self) -> (f32, f32, f32, f32) {
        match self.collision_type {
            CollisionType::Extrusion => (1., 0., 1., 1.),
            CollisionType::Blow { .. } => (1., 0., 0., 1.),
            CollisionType::Projectile { .. } => (0., 1., 0., 1.),
            CollisionType::Throw => (0., 0., 1., 1.),
            CollisionType::Damaged => (1., 1., 0., 1.),
        }
    }
}
