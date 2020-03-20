use crate::paramater::CollisionType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AnimationParam {
    pub collision_type: Option<CollisionType>,
}

impl AnimationParam {}
