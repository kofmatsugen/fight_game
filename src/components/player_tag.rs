use amethyst::ecs::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlayerTag {
    P1,
    P2,
}

impl Component for PlayerTag {
    type Storage = DenseVecStorage<Self>;
}
