use crate::{
    components::{Damaged, Knockback},
    paramater::FightTranslation,
};
use amethyst::{
    core::{math::Point2, Transform},
    ecs::{Entity, ReadStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba},
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
        ReadStorage<'s, Knockback>,
    );

    fn display(
        e: Entity,
        (time, key, transform, damaged, knockback): &Self::DisplayData,
    ) -> Option<String> {
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

        if let Some(knockback) = knockback.get(e) {
            out.push(format!("Knockback: {:.4} secs", knockback.knockback_time()));
        }

        if let Some(damaged) = damaged.get(e) {
            for id in damaged.damaged_ids() {
                out.push(format!("{:?}", id));
            }
        }

        Some(out.join("\n"))
    }

    fn debug_lines(
        e: Entity,
        debug_lines: &mut DebugLinesComponent,
        (_, _, transform, _, knockback): &Self::DisplayData,
        position_z: f32,
    ) -> Option<()> {
        let transform = transform.get(e)?;

        let translation = transform.translation();
        let base_y = translation.y;
        let base_x = translation.x;
        if let Some(knockback) = knockback.get(e) {
            if knockback.is_knockback() == true {
                let color = Srgba::new(1., 0., 1., 1.);

                let left_top = Point2::new(base_x, base_y + 10.);
                let right_down = Point2::new(base_x + knockback.knockback_time() * 600., base_y);

                debug_lines.add_rectangle_2d(left_top, right_down, position_z, color);
            }
        }
        Some(())
    }
}
