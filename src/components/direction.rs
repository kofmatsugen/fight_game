use amethyst::ecs::{Component, DenseVecStorage};

pub enum Direction {
    Right, // コマンド認識のデフォルト向き
    Left,
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}
