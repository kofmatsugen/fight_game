use crate::id::{file, pack};
use crate::paramater::AnimationParam;
use amethyst::{
    core::Transform,
    ecs::{Entity, ReadStorage},
};
use amethyst_sprite_studio::traits::translate_animation::TranslateAnimation;

#[derive(Debug)]
pub struct FightTranslation;

type FightFileId = file::FileId;
type FightPackKey = pack::PackKey;
type FightAnimationKey = pack::AnimationKey;
type FightUserData = AnimationParam;
type FightOptionalData<'s> = (ReadStorage<'s, Transform>,);

impl<'s> TranslateAnimation<'s> for FightTranslation {
    type FileId = FightFileId;
    type PackKey = FightPackKey;
    type AnimationKey = FightAnimationKey;
    type UserData = FightUserData;
    type OptionalData = FightOptionalData<'s>;

    fn translate_animation(
        entity: Entity,
        rest_time: f32,
        pack_anim_key: (&Self::PackKey, &Self::AnimationKey),
        user: Option<&Self::UserData>,
        optional: &Self::OptionalData,
    ) -> Option<(Self::PackKey, Self::AnimationKey, usize)> {
        if rest_time >= 0. {
            on_during_animation(entity, pack_anim_key, user, optional)
        } else {
            on_finish_animation(entity, pack_anim_key, user, optional)
        }
    }
}
// アニメーション中遷移判定
fn on_during_animation(
    _entity: Entity,
    (&_current_pack, &_current_anim): (&FightPackKey, &FightAnimationKey),
    _user: Option<&FightUserData>,
    _optional: &FightOptionalData,
) -> Option<(FightPackKey, FightAnimationKey, usize)> {
    None
}

// アニメーション終了時遷移判定
fn on_finish_animation(
    _entity: Entity,
    (&current_pack, &current_anim): (&FightPackKey, &FightAnimationKey),
    user: Option<&FightUserData>,
    _optional: &FightOptionalData,
) -> Option<(FightPackKey, FightAnimationKey, usize)> {
    let next_anim = match user {
        Some(_) => current_anim,
        None => pack::AnimationKey::Stance,
    };

    log::trace!("default next key: {:?}", (current_pack, next_anim, 0));

    Some((current_pack, next_anim, 0))
}
