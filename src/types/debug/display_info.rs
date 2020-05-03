use crate::paramater::FightTranslation;
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_sprite_studio::{
    components::{AnimationNodes, AnimationTime, PlayAnimationKey},
    traits::animation_file::AnimationFile,
};
use debug_system::traits::DebugDisplayFormat;

#[derive(Default)]
pub struct DisplayInfo;

impl<'s> DebugDisplayFormat<'s> for DisplayInfo {
    type DisplayData = (
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<FightTranslation>>,
        ReadStorage<'s, AnimationNodes<<FightTranslation as AnimationFile>::UserData>>,
    );

    fn display(e: Entity, (time, key, node): &Self::DisplayData) -> Option<String> {
        let mut out = Vec::new();
        let time = time.get(e)?;
        let key = key.get(e)?;
        let node = node.get(e)?;

        let (file, pack, anim) = key.play_key()?;

        out.push(format!("Key: {:?}/{:?}/{:?}", file, pack, anim));
        out.push(format!("Frame: {} F", node.play_frame()));

        match time {
            &AnimationTime::Play { .. } => {
                out.push(format!("Play"));
            }
            &AnimationTime::Stop { .. } => {
                out.push(format!("Stop"));
            }
        }

        Some(out.join("\n"))
    }
}
