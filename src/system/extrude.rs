use crate::components::ExtrudeCollision;
use amethyst::core::{
    components::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    math::{self, Isometry2, Vector2},
};
use ncollide2d::query;

pub struct ExtrudeSystem;

impl ExtrudeSystem {
    pub fn new() -> Self {
        ExtrudeSystem
    }
}

impl<'s> System<'s> for ExtrudeSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, ExtrudeCollision>,
    );

    fn run(&mut self, (entities, transform, extrusion): Self::SystemData) {
        for (e1, transform1, extrude1) in (&*entities, &transform, &extrusion).join() {
            if extrude1.shape().is_none() {
                continue;
            }
            let shape1 = extrude1.shape().unwrap();
            for (e2, transform2, extrude2) in (&*entities, &transform, &extrusion).join() {
                if extrude2.shape().is_none() {
                    continue;
                }
                let shape2 = extrude2.shape().unwrap();
                if e1 <= e2 {
                    continue;
                }
                if let Some(query::Contact { depth, normal, .. }) = query::contact(
                    &transform_to_isometry(transform1),
                    shape1.as_ref(),
                    &transform_to_isometry(transform2),
                    shape2.as_ref(),
                    0.0,
                ) {
                    log::info!("{:?}, {:?}", depth, normal);
                }
            }
        }
    }
}

fn transform_to_isometry(transform: &Transform) -> Isometry2<f32> {
    let matrix = transform.matrix();
    let collision: &[[f32; 4]; 4] = matrix.as_ref();
    Isometry2::new(Vector2::new(collision[3][0], collision[3][1]), math::zero())
}
