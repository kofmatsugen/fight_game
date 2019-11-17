use crate::paramater::CollisionType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyParamater {
    pub collision_type: Option<CollisionType>,
}
