use crate::paramater::FightTranslation;
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
    );

    fn display(e: Entity, (time, key, transform): &Self::DisplayData) -> Option<String> {
        let mut out = Vec::new();
        let time = time.get(e)?;
        let key = key.get(e)?;
        let transform = transform.get(e)?;

        let (file, pack, anim) = key.play_key()?;

        out.push(format!("Key: {:?}/{:?}/{:?}", file, pack, anim));

        match time {
            &AnimationTime::Play { current_time, .. } => {
                out.push(format!("Play: {:.3}", current_time));
            }
            &AnimationTime::Stop { stopped_time, .. } => {
                out.push(format!("Stop: {:.3}", stopped_time));
            }
        }

        out.push(format!(
            "Pos: ({:.2}, {:.2})",
            transform.translation().x,
            transform.translation().y
        ));

        Some(out.join("\n"))
    }
}
