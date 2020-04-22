use crate::components::Direction;
use amethyst::core::{
    components::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};
use movement_transform::components::Movement;

pub struct DirectionSystem;

impl DirectionSystem {
    pub fn new() -> Self {
        DirectionSystem
    }
}

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (transforms, direction, mut movements): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("direction");
        for (dir, transform, movement) in (&direction, &transforms, &mut movements).join() {
            let scale = transform.scale();

            // 左をデフォルトとして，現在のスケール値と合わせて左のときの正の値に変換する
            let current_dir_scale = scale.x.signum() * movement.transform().scale().x.signum();
            match dir {
                Direction::Right => {
                    // 右向きなので負の値ならそのまま，正の数なら反転
                    if current_dir_scale > 0. {
                        movement.transform_mut().scale_mut().x *= -1.;
                    }
                }
                Direction::Left => {
                    // 左向きなので正の値ならそのまま，正の数なら反転
                    if current_dir_scale < 0. {
                        movement.transform_mut().scale_mut().x *= -1.;
                    }
                }
            }
        }
    }
}
