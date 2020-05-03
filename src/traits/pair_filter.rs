use crate::{
    components::Damaged,
    paramater::{CollisionParamater, CollisionType},
    types::DamageCollisionId,
};
use amethyst::ecs::{Entity, ReadStorage};
use amethyst_aabb::traits::PairFilter;
use amethyst_sprite_studio::traits::animation_file::AnimationFile;

impl<'s, T> PairFilter<'s> for CollisionParamater<T>
where
    T: AnimationFile,
{
    type SystemData = (ReadStorage<'s, Damaged<T>>,);

    // 判定処理を行うフィルタ
    fn pair_filter(
        entity1: Entity,
        p1: &Self,
        entity2: Entity,
        p2: &Self,
        (damaged,): &Self::SystemData,
    ) -> bool {
        // どちらかが攻撃判定なら，すでにこの判定を食らっていないかチェック
        let filter = match (p1.collision_type, p2.collision_type) {
            // 押し出し判定は存在するならOK
            (CollisionType::Extrusion, CollisionType::Extrusion) => true,

            // ダメージvs攻撃なら，ダメージ側が攻撃側の判定とぶつかったことがないかチェック
            (CollisionType::Damaged, CollisionType::Blow { .. })
            | (CollisionType::Damaged, CollisionType::Projectile { .. })
            | (CollisionType::Damaged, CollisionType::Throw) => {
                yet_nothit_collision(entity1, damaged, p2.damage_collision_id.as_ref())
            }
            // 攻撃vsダメージなら，ダメージ側が攻撃側の判定とぶつかったことがないかチェック
            (CollisionType::Blow { .. }, CollisionType::Damaged)
            | (CollisionType::Projectile { .. }, CollisionType::Damaged)
            | (CollisionType::Throw, CollisionType::Damaged) => {
                yet_nothit_collision(entity2, damaged, p1.damage_collision_id.as_ref())
            }
            _ => false,
        };
        filter
    }
}

// まだぶつかってない判定かどうか
fn yet_nothit_collision<T>(
    entity: Entity,
    damaged: &ReadStorage<Damaged<T>>,
    id: Option<&DamageCollisionId<T>>,
) -> bool
where
    T: AnimationFile,
{
    damaged
        .get(entity)
        .and_then(|damaged| id.map(|id| (damaged, id)))
        .map(|(damaged, id)| damaged.contains(id) == false) // 含まれてなければぶつかってない
        .unwrap_or(true) // コンポーネントがないということはまだ何にもぶつかってない
}
