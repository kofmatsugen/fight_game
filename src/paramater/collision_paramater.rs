use crate::{
    paramater::{AnimationParam, CollisionType},
    traits::ParamaterFromData,
};

#[derive(Clone, Copy, Debug)]
pub struct CollisionParamater {
    _collision_type: CollisionType,
}

impl ParamaterFromData<AnimationParam> for CollisionParamater {
    fn make_collision_data(param: Option<&AnimationParam>) -> Option<Self> {
        let _collision_type = param?.collision_type?;

        Some(CollisionParamater { _collision_type })
    }
}
