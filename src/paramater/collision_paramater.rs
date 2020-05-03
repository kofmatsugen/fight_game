use crate::{
    components::SkillCount,
    paramater::{AnimationParam, CollisionType},
    traits::{ExtrudeFilter, ParamaterFromData},
    types::DamageCollisionId,
};
use amethyst::ecs::{Entity, ReadStorage};
#[cfg(feature = "debug")]
use amethyst_aabb::debug::traits::CollisionColor;
use amethyst_sprite_studio::{components::PlayAnimationKey, traits::animation_file::AnimationFile};

#[derive(Clone, Copy, Debug)]
pub struct CollisionParamater<T>
where
    T: AnimationFile,
{
    pub collision_type: CollisionType,
    pub damage_collision_id: Option<DamageCollisionId<T>>,
}

impl<'s, T> ParamaterFromData<'s, AnimationParam> for CollisionParamater<T>
where
    T: AnimationFile,
{
    type SystemData = (
        // 判定ID生成に必要
        ReadStorage<'s, PlayAnimationKey<T>>,
        ReadStorage<'s, SkillCount<T>>,
    );
    fn make_collision_data(
        entity: Entity,
        param: Option<&AnimationParam>,
        (keys, skill_counts): &Self::SystemData,
    ) -> Option<Self> {
        let collision_type = param?.collision_type?;

        let damage_collision_id = match &collision_type {
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
            damage_collision_id,
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
