use crate::id::{command::Command, skill::SkillId};
use amethyst::ecs::{Component, DenseVecStorage};
use std::collections::BTreeMap;

pub struct SkillSet {
    _skills: BTreeMap<Command, SkillId>,
}

impl Component for SkillSet {
    type Storage = DenseVecStorage<Self>;
}
