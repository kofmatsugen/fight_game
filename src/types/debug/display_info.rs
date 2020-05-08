use crate::{components::Damaged, paramater::FightTranslation};
use amethyst::{
    core::Transform,
    ecs::{Entity, ReadStorage},
};
use amethyst_sprite_studio::components::{AnimationTime, PlayAnimationKey};
use debug_system::traits::DebugDisplayFormat;

#[derive(Default)]
pub struct DisplayInfo;

impl<'s> DebugDisplayFormat<'s> for DisplayInfo {
    type DisplayData = (
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<FightTranslation>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Damaged<FightTranslation>>,
    );

    fn display(e: Entity, (time, key, transform, damaged): &Self::DisplayData) -> Option<String> {
        let mut out = Vec::new();
        let time = time.get(e)?;
        let key = key.get(e)?;
        let transform = transform.get(e)?;

        let (file, pack, anim) = key.play_key()?;

        out.push(format!("Key: {:?}/{:?}/{:?}", file, pack, anim));

        let played = if time.is_play() == true {
            "Play"
        } else {
            "Stop"
        };

        let current_frame = time.play_frame(60.);

        out.push(format!(
            "{}: {:3} F, {:6.3} ms",
            played,
            current_frame,
            time.play_time() * 1000.
        ));
        out.push(format!(
            "Pos: ({:.2}, {:.2})",
            transform.translation().x,
            transform.translation().y
        ));

        if let Some(damaged) = damaged.get(e) {
            for id in damaged.damaged_ids() {
                out.push(format!("{:?}", id));
            }
        }

        Some(out.join("\n"))
    }
}
