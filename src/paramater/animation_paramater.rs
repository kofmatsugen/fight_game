use crate::{flag::cancel::Cancel, paramater::CollisionType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AnimationParam {
    pub collision_type: Option<CollisionType>,
    #[serde(default, skip_serializing_if = "Cancel::is_empty")]
    pub cancel: Cancel,
}

impl AnimationParam {}
