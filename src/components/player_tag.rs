use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, PrefabData,
)]
#[prefab(Component)]
pub enum PlayerTag {
    P1,
    P2,
}

impl Component for PlayerTag {
    type Storage = DenseVecStorage<Self>;
}
