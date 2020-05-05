use crate::{
    components::Damaged,
    paramater::{CollisionParamater, CollisionType},
    types::DamageCollisionId,
};
use amethyst::ecs::Entity;
use amethyst_aabb::collide::pipeline::{
    broad_phase::BroadPhasePairFilter,
    object::{CollisionObject as NCollideObject, CollisionObjectSlabHandle},
};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use std::marker::PhantomData;

pub struct FightPairFilter<T> {
    marker: PhantomData<T>,
}

impl<T>
    BroadPhasePairFilter<
        f32,
        NCollideObject<f32, (Entity, CollisionParamater<T>)>,
        CollisionObjectSlabHandle,
    > for FightPairFilter<T>
where
    T: AnimationFile,
{
    fn is_pair_valid(
        &self,
        b1: &NCollideObject<f32, (Entity, CollisionParamater<T>)>,
        b2: &NCollideObject<f32, (Entity, CollisionParamater<T>)>,
        _: CollisionObjectSlabHandle,
        _: CollisionObjectSlabHandle,
    ) -> bool {
        let (e1, p1) = &b1.data();
        let (e2, p2) = &b2.data();
        if e1 == e2 {
            return false;
        }
        match (p1.collision_type, p2.collision_type) {
            // 押し出し判定は存在するならOK
            (CollisionType::Extrusion, CollisionType::Extrusion) => true,

            // ダメージvs攻撃なら，ダメージ側が攻撃側の判定とぶつかったことがないかチェック
            (CollisionType::Damaged, CollisionType::Blow { .. })
            | (CollisionType::Damaged, CollisionType::Projectile { .. })
            | (CollisionType::Damaged, CollisionType::Throw) => {
                yet_nothit_collision(p2.collision_id.as_ref(), p1.damaged_collision_ids.as_ref())
            }
            // 攻撃vsダメージなら，ダメージ側が攻撃側の判定とぶつかったことがないかチェック
            (CollisionType::Blow { .. }, CollisionType::Damaged)
            | (CollisionType::Projectile { .. }, CollisionType::Damaged)
            | (CollisionType::Throw, CollisionType::Damaged) => {
                yet_nothit_collision(p1.collision_id.as_ref(), p2.damaged_collision_ids.as_ref())
            }
            _ => false,
        }
    }
}

// まだぶつかってない判定かどうか
fn yet_nothit_collision<T>(
    attack_id: Option<&DamageCollisionId<T>>,
    damaged_ids: Option<&Damaged<T>>,
) -> bool
where
    T: AnimationFile,
{
    damaged_ids
        .and_then(|ids| attack_id.map(|id| (ids, id)))
        .map(|(ids, id)| ids.contains(id) == false)
        .unwrap_or(true)
}

impl<T> Default for FightPairFilter<T> {
    fn default() -> Self {
        FightPairFilter {
            marker: PhantomData,
        }
    }
}
