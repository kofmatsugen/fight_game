use crate::traits::CollisionData;
use ncollide2d::{
    math::Vector,
    shape::{Cuboid, ShapeHandle},
};

#[derive(Debug)]
pub struct Aabb {
    width: f32,
    height: f32,
}

impl Aabb {
    pub fn new(width: f32, height: f32) -> Self {
        Aabb {
            width: width.abs(),
            height: height.abs(),
        }
    }
}

impl CollisionData for Aabb {
    fn make_shape(&self) -> ShapeHandle<f32> {
        ShapeHandle::new(Cuboid::new(Vector::new(self.width / 2., self.height / 2.)))
    }
}
