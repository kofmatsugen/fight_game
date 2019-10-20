use amethyst::ecs::{Component, DenseVecStorage};
use ncollide2d::shape::{Shape, ShapeHandle};

// 押し出し判定
pub struct ExtrudeCollision {
    shape: Option<ShapeHandle<f32>>,
}

impl ExtrudeCollision {
    pub fn new<S: Shape<f32>>(shape: S) -> Self {
        ExtrudeCollision {
            shape: Some(ShapeHandle::new(shape)),
        }
    }

    pub fn none() -> Self {
        ExtrudeCollision { shape: None }
    }

    pub fn shape(&self) -> Option<&ShapeHandle<f32>> {
        self.shape.as_ref()
    }
}

impl Component for ExtrudeCollision {
    type Storage = DenseVecStorage<Self>;
}
