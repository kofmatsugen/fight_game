use crate::{
    components::ActiveCommand,
    id::{command, file, pack},
    paramater::AnimationParam,
};
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_sprite_studio::traits::{
    animation_file::AnimationFile, translate_animation::TranslateAnimation,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct FightTranslation;

type FightFileId = file::FileId;
type FightPackKey = pack::PackKey;
type FightAnimationKey = pack::AnimationKey;
type FightUserData = AnimationParam;
type FightOptionalData<'s> = (ReadStorage<'s, ActiveCommand>,);
impl AnimationFile for FightTranslation {
    type FileId = FightFileId;
    type PackKey = FightPackKey;
    type AnimationKey = FightAnimationKey;
    type UserData = FightUserData;

    fn to_file_name(file_id: &Self::FileId) -> &'static str {
        FILE_LIST[file_id].0
    }

    fn sprite_sheet_num(file_id: &Self::FileId) -> usize {
        FILE_LIST[file_id].1
    }
}

impl<'s> TranslateAnimation<'s> for FightTranslation {
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

lazy_static::lazy_static! {
    static ref FILE_LIST: BTreeMap<file::FileId, (&'static str, usize)> = {
        let mut list = BTreeMap::new();
        list.insert(file::FileId::SpriteStudioSplash, ("splash1024", 1));
        list.insert(file::FileId::Sample, ("sample", 1));
        list
    };
}
