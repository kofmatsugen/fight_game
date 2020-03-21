use crate::{
    components::ActiveCommand,
    id::{command, file, pack},
    paramater::AnimationParam,
};
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_sprite_studio::traits::translate_animation::TranslateAnimation;

#[derive(Debug)]
pub struct FightTranslation;

type FightFileId = file::FileId;
type FightPackKey = pack::PackKey;
type FightAnimationKey = pack::AnimationKey;
type FightUserData = AnimationParam;
type FightOptionalData<'s> = (ReadStorage<'s, ActiveCommand>,);

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
            Some(on_finish_animation(entity, pack_anim_key, user, optional))
        }
    }
}
// アニメーション中遷移判定
fn on_during_animation(
    e: Entity,
    (&current_pack, current_anim): (&FightPackKey, &FightAnimationKey),
    _user: Option<&FightUserData>,
    (active_commands,): &FightOptionalData,
) -> Option<(FightPackKey, FightAnimationKey, usize)> {
    // 終了したら基本初期に戻る
    let active_command = active_commands.get(e)?;

    // とりあえずenum値的に最大値を優先する
    // 遷移ルールも含めて最終的にはデータ側に移動したい
    let next = match active_command.active_commands().max() {
        Some(command::Command::Back) => None,
        Some(command::Command::Walk) => {
            if current_anim == &pack::AnimationKey::Run {
                None
            } else {
                None
            }
        }
        Some(command::Command::BackDash) => None,
        Some(command::Command::Dash) => Some(pack::AnimationKey::Run),
        Some(command::Command::VerticalJump) => None,
        Some(command::Command::BackJump) => None,
        Some(command::Command::FrontJump) => None,
        Some(command::Command::Crouch) => {
            if current_anim == &pack::AnimationKey::Sit
                || current_anim == &pack::AnimationKey::Sitdown
            {
                Some(pack::AnimationKey::Sit)
            } else {
                Some(pack::AnimationKey::Sitdown)
            }
        }
        Some(command::Command::BackCrouch) => {
            if current_anim == &pack::AnimationKey::Sit
                || current_anim == &pack::AnimationKey::Sitdown
            {
                Some(pack::AnimationKey::Sit)
            } else {
                Some(pack::AnimationKey::Sitdown)
            }
        }
        Some(command::Command::FrontCrouch) => {
            if current_anim == &pack::AnimationKey::Sit
                || current_anim == &pack::AnimationKey::Sitdown
            {
                Some(pack::AnimationKey::Sit)
            } else {
                Some(pack::AnimationKey::Sitdown)
            }
        }
        Some(command::Command::A) => Some(pack::AnimationKey::Punch1),
        Some(command::Command::B) => Some(pack::AnimationKey::Kick1),
        Some(command::Command::C) => Some(pack::AnimationKey::Punch2),
        Some(command::Command::D) => Some(pack::AnimationKey::Kick2),
        None => None,
    }?;

    if &next == current_anim {
        None
    } else {
        Some((current_pack, next, 0))
    }
}

// アニメーション終了時遷移判定
fn on_finish_animation(
    e: Entity,
    (&current_pack, current_anim): (&FightPackKey, &FightAnimationKey),
    _user: Option<&FightUserData>,
    (active_commands,): &FightOptionalData,
) -> (FightPackKey, FightAnimationKey, usize) {
    // 終了したら基本初期に戻る
    let active_command = active_commands.get(e);

    // とりあえずenum値的に最大値を優先する
    // 遷移ルールも含めて最終的にはデータ側に移動したい
    let next = active_command
        .and_then(|c| match c.active_commands().max() {
            Some(command::Command::Back) => None,
            Some(command::Command::Walk) => {
                if current_anim == &pack::AnimationKey::Run {
                    Some(pack::AnimationKey::Run)
                } else {
                    Some(pack::AnimationKey::Walk)
                }
            }
            Some(command::Command::BackDash) => None,
            Some(command::Command::Dash) => Some(pack::AnimationKey::Run),
            Some(command::Command::VerticalJump) => None,
            Some(command::Command::BackJump) => None,
            Some(command::Command::FrontJump) => None,
            Some(command::Command::Crouch) => {
                if current_anim == &pack::AnimationKey::Sit
                    || current_anim == &pack::AnimationKey::Sitdown
                {
                    Some(pack::AnimationKey::Sit)
                } else {
                    Some(pack::AnimationKey::Sitdown)
                }
            }
            Some(command::Command::BackCrouch) => {
                if current_anim == &pack::AnimationKey::Sit
                    || current_anim == &pack::AnimationKey::Sitdown
                {
                    Some(pack::AnimationKey::Sit)
                } else {
                    Some(pack::AnimationKey::Sitdown)
                }
            }
            Some(command::Command::FrontCrouch) => {
                if current_anim == &pack::AnimationKey::Sit
                    || current_anim == &pack::AnimationKey::Sitdown
                {
                    Some(pack::AnimationKey::Sit)
                } else {
                    Some(pack::AnimationKey::Sitdown)
                }
            }
            Some(command::Command::A) => Some(pack::AnimationKey::Punch1),
            Some(command::Command::B) => Some(pack::AnimationKey::Kick1),
            Some(command::Command::C) => Some(pack::AnimationKey::Punch2),
            Some(command::Command::D) => Some(pack::AnimationKey::Kick2),
            None => None,
        })
        .unwrap_or(pack::AnimationKey::Stance);

    (current_pack, next, 0)
}
