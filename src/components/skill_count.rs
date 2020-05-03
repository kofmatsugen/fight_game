use amethyst::ecs::{Component, DenseVecStorage};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::collections::BTreeMap;

// 使用したスキルの回数カウント
// 主に判定のID指定に使用する
pub struct SkillCount<T>
where
    T: AnimationFile,
{
    counts: BTreeMap<(T::FileId, T::PackKey, T::AnimationKey), u64>,
}

impl<T> SkillCount<T>
where
    T: AnimationFile,
{
    pub fn new() -> Self {
        SkillCount {
            counts: BTreeMap::new(),
        }
    }

    pub fn increment(&mut self, key: (T::FileId, T::PackKey, T::AnimationKey)) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub fn skill_count(&self, key: &(T::FileId, T::PackKey, T::AnimationKey)) -> u64 {
        self.counts.get(key).map(|count| *count).unwrap_or(0)
    }
}

impl<T> Component for SkillCount<T>
where
    T: AnimationFile,
{
    type Storage = DenseVecStorage<Self>;
}
