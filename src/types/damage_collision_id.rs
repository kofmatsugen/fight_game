use amethyst::ecs::Entity;
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::cmp::Ordering;

// ダメージを受けたときの判定のID
// 主に判定の重複ヒットを回避する
// IDは，下記の情報で一意になるはず
// ・判定を持っていたエンティティ
// ・判定を所持していたアニメーションキー
// ・任意で追加されるアニメーション内の判定ID
// ・同一のアニメーションの使用回数
#[derive(Hash)]
pub struct DamageCollisionId<T>
where
    T: AnimationFile,
{
    collision_owner: Entity,
    file_id: T::FileId,
    pack: T::PackKey,
    animation: T::AnimationKey,
    collision_id: u32,
    animation_count: u64,
}

impl<T> DamageCollisionId<T>
where
    T: AnimationFile,
{
    pub fn new(
        &(collision_owner, file_id, pack, animation, collision_id, animation_count): &(
            Entity,
            T::FileId,
            T::PackKey,
            T::AnimationKey,
            u32,
            u64,
        ),
    ) -> Self {
        DamageCollisionId {
            collision_owner,
            file_id,
            pack,
            animation,
            collision_id,
            animation_count,
        }
    }
}

impl<T> PartialOrd for DamageCollisionId<T>
where
    T: AnimationFile,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for DamageCollisionId<T>
where
    T: AnimationFile,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.collision_owner.cmp(&other.collision_owner) != Ordering::Equal {
            self.collision_owner.cmp(&other.collision_owner)
        } else if self.file_id.cmp(&other.file_id) != Ordering::Equal {
            self.file_id.cmp(&other.file_id)
        } else if self.pack.cmp(&other.pack) != Ordering::Equal {
            self.pack.cmp(&other.pack)
        } else if self.animation.cmp(&other.animation) != Ordering::Equal {
            self.animation.cmp(&other.animation)
        } else if self.collision_id.cmp(&other.collision_id) != Ordering::Equal {
            self.collision_id.cmp(&other.collision_id)
        } else if self.animation_count.cmp(&other.animation_count) != Ordering::Equal {
            self.animation_count.cmp(&other.animation_count)
        } else {
            Ordering::Equal
        }
    }
}

impl<T> PartialEq for DamageCollisionId<T>
where
    T: AnimationFile,
{
    fn eq(&self, other: &Self) -> bool {
        self.collision_owner == other.collision_owner
            && self.file_id == other.file_id
            && self.pack == other.pack
            && self.animation == other.animation
            && self.collision_id == other.collision_id
            && self.animation_count == other.animation_count
    }
}

impl<T> Eq for DamageCollisionId<T> where T: AnimationFile {}

impl<T> Clone for DamageCollisionId<T>
where
    T: AnimationFile,
{
    fn clone(&self) -> Self {
        DamageCollisionId {
            collision_owner: self.collision_owner,
            file_id: self.file_id,
            pack: self.pack,
            animation: self.animation,
            collision_id: self.collision_id,
            animation_count: self.animation_count,
        }
    }
}

impl<T> Copy for DamageCollisionId<T> where T: AnimationFile {}

impl<T> std::fmt::Debug for DamageCollisionId<T>
where
    T: AnimationFile,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}{:?}{:?}c{:03}i{:03}",
            self.file_id, self.pack, self.animation, self.animation_count, self.collision_id
        ))
    }
}
