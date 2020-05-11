use crate::paramater::{CollisionType, FightTranslation};
use amethyst_sprite_studio::{
    resource::data::AnimationData, traits::animation_file::AnimationFile,
};
use std::collections::BTreeSet;

type PackKey = <FightTranslation as AnimationFile>::PackKey;
type AnimationKey = <FightTranslation as AnimationFile>::AnimationKey;

// アニメーションをパースして技の情報を取得する
#[derive(Debug)]
pub struct SkillInfomation {
    attack_frames: Vec<(usize, usize)>, // 本体付随の攻撃判定が存在するフレーム
}

impl SkillInfomation {
    pub fn make_info(
        animation: &AnimationData<FightTranslation>,
        pack: PackKey,
        anim: AnimationKey,
    ) -> Option<Self> {
        let pack = animation.pack(&pack)?;
        let parts_num = pack.parts().count();
        let animation = pack.animation(&anim)?;
        let mut attack_frame_set = BTreeSet::new();
        for f in 0..animation.total_frame() {
            for p in 0..parts_num {
                if animation.hide(p, f) == true {
                    continue;
                }
                if let Some(user) = animation.user(p, f) {
                    match &user.collision_type {
                        &Some(CollisionType::Blow { .. })
                        | &Some(CollisionType::Projectile { .. }) => {
                            attack_frame_set.insert(f);
                        }
                        _ => {}
                    }
                }
            }
        }

        log::info!("test: collisions: {:?}", attack_frame_set);

        let mut attack_frames = vec![];
        let mut start = None;
        let mut end = None;
        // 連続するフレームを取り出す
        // 0,1,2,3 => (0,3)
        // 0,1,2,5,6,7 => (0,2), (5,7)
        for f in attack_frame_set {
            if start.is_none() == true {
                start = Some(f);
            }
            match end {
                Some(end_frame) => {
                    if f - end_frame == 1 {
                        // 間は空いてないので終端はまだ
                        log::info!("set end: {}", f);
                        end = Some(f);
                    } else {
                        // 間が空いているので一旦区切る
                        attack_frames.push((start.unwrap(), end_frame));
                        start = None;
                        end = None;
                    }
                }
                None => end = Some(f),
            }
        }
        // まだセットしてないフレームが有る場合はそれをセット
        match (start, end) {
            (Some(start), Some(end)) => {
                attack_frames.push((start, end));
            }
            _ => {}
        }

        Some(SkillInfomation { attack_frames })
    }
}
