use crate::id::{command::Command, pack::AnimationKey};
use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct SkillSet {
    neutral: AnimationKey,
    skills: BTreeMap<Command, AnimationKey>,
}

impl SkillSet {
    pub fn neutral_skill(&self) -> &AnimationKey {
        &self.neutral
    }

    pub fn command_skill(&self, command: &Command) -> Option<&AnimationKey> {
        self.skills.get(command)
    }
}

impl Component for SkillSet {
    type Storage = DenseVecStorage<Self>;
}
