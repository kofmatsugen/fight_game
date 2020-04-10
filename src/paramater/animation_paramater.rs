use crate::{flag::cancel::Cancel, paramater::CollisionType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AnimationParam {
    pub collision_type: Option<CollisionType>,
    #[serde(default)]
    pub cancel: Cancel,
}

impl AnimationParam {}
