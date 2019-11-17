use crate::traits::CollisionData;
use ncollide2d::{
    math::Vector,
    shape::{Cuboid, ShapeHandle},
};

pub struct Aabb {
    width: f32,
    height: f32,
}

impl CollisionData for Aabb {
    fn make_shape(&self) -> ShapeHandle<f32> {
        ShapeHandle::new(Cuboid::new(Vector::new(self.width / 2., self.height / 2.)))
    }
}
