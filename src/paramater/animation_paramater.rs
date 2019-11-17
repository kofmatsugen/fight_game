use crate::paramater::KeyParamater;
use amethyst_sprite_studio::traits::{CollisionColor, FromUser};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationParam {
    pub move_direction: [f32; 2],
    pub key_param: Option<KeyParamater>,
}

impl AnimationParam {
    pub fn move_direction(&self) -> [f32; 2] {
        self.move_direction
    }
}
impl FromUser for AnimationParam {
    fn from_user(
        _integer: Option<i32>,
        point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        text: Option<String>,
    ) -> Option<Self> {
        let move_direction = point.map(|(x, y)| [x, y]).unwrap_or([0., 0.]);
        let key_param =
            text.and_then(
                |string| match serde_json::de::from_str::<KeyParamater>(&string) {
                    Ok(user_param) => user_param.into(),
                    Err(err) => {
                        log::error!("{:?}", err);
                        log::error!("raw: {:?}", string);
                        None
                    }
                },
            );

        AnimationParam {
            move_direction,
            key_param,
        }
        .into()
    }
}

impl CollisionColor for AnimationParam {}
