use amethyst_sprite_studio::traits::{CollisionColor, FromUser};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationParam {
    move_direction: [f32; 2],
}

impl FromUser for AnimationParam {
    fn from_user(
        _integer: Option<i32>,
        point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Option<Self> {
        point.map(|(x, y)| AnimationParam {
            move_direction: [x, y],
        })
    }
}

impl CollisionColor for AnimationParam {}
