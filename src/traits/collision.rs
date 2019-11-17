use ncollide2d::shape::ShapeHandle;

pub trait CollisionData {
    fn make_shape(&self) -> ShapeHandle<f32>;
}
