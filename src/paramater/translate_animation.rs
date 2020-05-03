use crate::{
    components::{ActiveCommand, SkillSet},
    id::{file, pack},
    paramater::AnimationParam,
};
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_sprite_studio::traits::{
    animation_file::AnimationFile, translate_animation::TranslateAnimation,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct FightTranslation;

type FightFileId = file::FileId;
type FightPackKey = pack::PackKey;
type FightAnimationKey = pack::AnimationKey;
type FightUserData = AnimationParam;
type FightOptionalData<'s> = (ReadStorage<'s, ActiveCommand>, ReadStorage<'s, SkillSet>);
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
        (active_commands, skill_sets): &Self::OptionalData,
    ) -> Option<(Self::PackKey, Self::AnimationKey, usize)> {
        let active = active_commands.get(entity)?;
        let skill_set = skill_sets.get(entity)?;
        let next = if rest_time >= 0. {
            on_during_animation(pack_anim_key, user, active, skill_set)
        } else {
            on_finish_animation(pack_anim_key, user, active, skill_set)
        };
        log::trace!("=> {:?}", next);
        next
    }
}
// アニメーション中遷移判定
// 遷移ルールも含めて最終的にはデータ側に移動したい
fn on_during_animation(
    (&current_pack, current_anim): (&FightPackKey, &FightAnimationKey),
    user: Option<&FightUserData>,
    active: &ActiveCommand,
    skill_set: &SkillSet,
) -> Option<(FightPackKey, FightAnimationKey, usize)> {
    // とりあえずenum値的に最大値を優先する
    let command = active.active_commands().max()?;
    // 行動中はそのアニメーションパラメータ内のキャンセルフラグで判定
    if user?.cancel.is_cancelable(command) == false {
        return None;
    }
    log::debug!("canceled: {:?}", command);

    let skill = skill_set.command_skill(command)?;
    if skill == current_anim {
        None
    } else {
        Some((current_pack, *skill, 0))
    }
}

// アニメーション終了時遷移判定
fn on_finish_animation(
    (&current_pack, _current_anim): (&FightPackKey, &FightAnimationKey),
    _user: Option<&FightUserData>,
    active: &ActiveCommand,
    skill_set: &SkillSet,
) -> Option<(FightPackKey, FightAnimationKey, usize)> {
    // とりあえずenum値的に最大値を優先する
    // 遷移ルールも含めて最終的にはデータ側に移動したい
    let command = active.active_commands().max();
    let skill = command
        .and_then(|command| skill_set.command_skill(command))
        .unwrap_or(skill_set.neutral_skill());
    Some((current_pack, *skill, 0))
}

lazy_static::lazy_static! {
    static ref FILE_LIST: BTreeMap<file::FileId, (&'static str, usize)> = {
        let mut list = BTreeMap::new();
        list.insert(file::FileId::Sample, ("sample", 1));
        list.insert(file::FileId::Sandbox, ("sandbox", 1));
        list
    };
}
