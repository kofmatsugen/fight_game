use crate::paramater::CollisionType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationParam {
    pub collision_type: Option<CollisionType>,
}

impl AnimationParam {}
