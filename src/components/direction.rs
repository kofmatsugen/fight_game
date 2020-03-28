use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub enum Direction {
    Right, // コマンド認識のデフォルト向き
    Left,
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}
