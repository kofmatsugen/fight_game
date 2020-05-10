use crate::{
    flag::Cancel,
    paramater::{ChangeParamater, CollisionType, FightTranslation},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationParam {
    pub collision_type: Option<CollisionType>,
    #[serde(default, skip_serializing_if = "Cancel::is_empty")]
    pub cancel: Cancel,
    pub change: Option<ChangeParamater<FightTranslation>>,
}

impl AnimationParam {}
