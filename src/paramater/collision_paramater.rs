use crate::{
    components::{Damaged, SkillCount},
    paramater::{AnimationParam, CollisionType},
    traits::{ExtrudeFilter, ParamaterFromData},
    types::DamageCollisionId,
};
use amethyst::ecs::{Entity, ReadStorage};
#[cfg(feature = "debug")]
use amethyst_aabb::debug::traits::CollisionColor;
use amethyst_aabb::traits::CollisionObject;
use amethyst_sprite_studio::{components::PlayAnimationKey, traits::animation_file::AnimationFile};

#[derive(Debug)]
pub struct CollisionParamater<T>
where
    T: AnimationFile,
{
    pub collision_type: CollisionType,
    pub collision_id: Option<DamageCollisionId<T>>,
    pub damaged_collision_ids: Option<Damaged<T>>,
}

impl<T> CollisionObject for CollisionParamater<T>
where
    T: AnimationFile,
{
    fn pair_filter(p1: &Self, p2: &Self) -> bool {
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

impl<'s, T> ParamaterFromData<'s, AnimationParam> for CollisionParamater<T>
where
    T: AnimationFile,
{
    type SystemData = (
        // 判定ID生成に必要
        ReadStorage<'s, PlayAnimationKey<T>>,
        ReadStorage<'s, SkillCount<T>>,
        ReadStorage<'s, Damaged<T>>,
    );
    fn make_collision_data(
        entity: Entity,
        param: Option<&AnimationParam>,
        (keys, skill_counts, damaged): &Self::SystemData,
    ) -> Option<Self> {
        let collision_type = param?.collision_type?;

        let collision_id = match &collision_type {
            &CollisionType::Blow {
                collision_count, ..
            }
            | &CollisionType::Projectile {
                collision_count, ..
            } => {
                // 攻撃判定だったら判定IDを作る
                let (&file, &pack, &anim) = keys.get(entity)?.play_key()?;
                let count = skill_counts
                    .get(entity)
                    .map(|count| count.skill_count(&(file, pack, anim)))
                    .unwrap_or(0);
                Some(DamageCollisionId::new(&(
                    entity,
                    file,
                    pack,
                    anim,
                    collision_count,
                    count,
                )))
            }
            _ => None,
        };

        Some(CollisionParamater {
            collision_type,
            collision_id,
            damaged_collision_ids: damaged.get(entity).cloned(),
        })
    }
}

impl<'s, T> ExtrudeFilter<'s> for CollisionParamater<T>
where
    T: AnimationFile,
{
    type SystemData = ();

    // 押し出し判定を行うフィルタ
    fn extrude_filter(
        _entity1: Entity,
        p1: &Self,
        _entity2: Entity,
        p2: &Self,
        _data: &Self::SystemData,
    ) -> bool {
        match (p1.collision_type, p2.collision_type) {
            (CollisionType::Extrusion, CollisionType::Extrusion) => true,
            _ => false,
        }
    }
}

#[cfg(feature = "debug")]
impl<T> CollisionColor for CollisionParamater<T>
where
    T: AnimationFile,
{
    fn collision_color(&self) -> (f32, f32, f32, f32) {
        match self.collision_type {
            CollisionType::Extrusion => (1., 0., 1., 1.),
            CollisionType::Blow { .. } => (1., 0., 0., 1.),
            CollisionType::Projectile { .. } => (0., 1., 0., 1.),
            CollisionType::Throw => (0., 0., 1., 1.),
            CollisionType::Damaged => (1., 1., 0., 1.),
        }
    }
}

impl<T> Clone for CollisionParamater<T>
where
    T: AnimationFile,
{
    fn clone(&self) -> Self {
        CollisionParamater {
            collision_type: self.collision_type,
            collision_id: self.collision_id,
            damaged_collision_ids: self.damaged_collision_ids.clone(),
        }
    }
}
