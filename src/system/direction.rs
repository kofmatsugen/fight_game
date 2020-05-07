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
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("direction");
        for (dir, transform) in (&direction, &mut transforms).join() {
            let scale = transform.scale();

            // 左をデフォルトとして，現在のスケール値と合わせて左のときの正の値に変換する
            let current_dir_scale = scale.x.signum();
            match dir {
                Direction::Right => {
                    // 右向きなので負の値ならそのまま，正の数なら反転
                    if current_dir_scale > 0. {
                        transform.scale_mut().x *= -1.;
                    }
                }
                Direction::Left => {
                    // 左向きなので正の値ならそのまま，正の数なら反転
                    if current_dir_scale < 0. {
                        transform.scale_mut().x *= -1.;
                    }
                }
            }
        }
    }
}
