use crate::{
    paramater::{AnimationParam, CollisionType},
    traits::ParamaterFromData,
};
#[cfg(feature = "debug")]
use amethyst_collision::traits::debug::collision_color::CollisionColor;

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

#[cfg(feature = "debug")]
impl CollisionColor for CollisionParamater {
    fn collision_color(&self) -> (f32, f32, f32, f32) {
        match self.collision_type {
            CollisionType::Extrusion => (1., 0., 1., 1.),
            CollisionType::Blow { .. } => (1., 0., 0., 1.),
            CollisionType::Projectile { .. } => (0., 1., 0., 1.),
            CollisionType::Throw => (0., 0., 1., 1.),
        }
    }
}
