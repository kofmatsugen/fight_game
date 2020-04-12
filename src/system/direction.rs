use crate::components::Direction;
use amethyst::core::{
    components::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub struct DirectionSystem;

impl DirectionSystem {
    pub fn new() -> Self {
        DirectionSystem
    }
}

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Direction>);

    fn run(&mut self, (mut transforms, direction): Self::SystemData) {
        for (dir, transform) in (&direction, &mut transforms).join() {
            let scale = transform.scale();
            // アニメーションのデフォルトは左向き
            let dir_x = match dir {
                Direction::Right => f32::abs(scale.x) * -1.,
                Direction::Left => f32::abs(scale.x),
            };

            transform.scale_mut().x = dir_x;
        }
    }
}
