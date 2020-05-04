use crate::types::DamageCollisionId;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::collections::BTreeSet;

// 受けたダメージの判定ID
// ノックバック終了時にクリアされる．
// 受けた判定IDと同じ攻撃は受け付けない
pub struct Damaged<T>
where
    T: AnimationFile,
{
    damaged_collision_ids: BTreeSet<DamageCollisionId<T>>,
}

impl<T> Damaged<T>
where
    T: AnimationFile,
{
    pub(crate) fn new() -> Self {
        Damaged {
            damaged_collision_ids: BTreeSet::<DamageCollisionId<T>>::new(),
        }
    }

    pub(crate) fn add_id(&mut self, id: DamageCollisionId<T>) {
        self.damaged_collision_ids.insert(id);
        log::trace!("inserted: {:?}", self.damaged_collision_ids);
    }

    pub(crate) fn contains(&self, id: &DamageCollisionId<T>) -> bool {
        log::trace!("{:?} contains {:?}", id, self.damaged_collision_ids);
        self.damaged_collision_ids.contains(id)
    }

    pub(crate) fn clear(&mut self) {
        log::trace!("clear: {:?}", self.damaged_collision_ids);
        self.damaged_collision_ids.clear();
    }
}

impl<T> Component for Damaged<T>
where
    T: AnimationFile,
{
    type Storage = DenseVecStorage<Self>;
}
