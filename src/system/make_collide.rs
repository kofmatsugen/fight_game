use crate::{
    components::ExtrudeCollision,
    paramater::{AnimationParam, CollisionType, UserParamater},
};
use amethyst::{
    assets::AssetStorage,
    core::{
        components::Transform,
        ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
        math::{self, Isometry2, Vector2},
    },
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    iter::AnimationNodes,
    resource::AnimationStore,
    traits::AnimationKey,
    SpriteAnimation,
};
use ncollide2d::shape::{Compound, Cuboid, ShapeHandle};
use std::{collections::BTreeMap, marker::PhantomData};

pub struct CollideSystem<K> {
    _key: PhantomData<K>,
}

impl<K> CollideSystem<K> {
    pub fn new() -> Self {
        CollideSystem { _key: PhantomData }
    }
}

impl<'s, K> System<'s> for CollideSystem<K>
where
    K: AnimationKey,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, AnimationStore<K, AnimationParam>>,
        Read<'s, AssetStorage<SpriteAnimation<AnimationParam>>>,
        ReadStorage<'s, PlayAnimationKey<K>>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, ExtrudeCollision>,
    );

    fn run(
        &mut self,
        (entities, store, storage, keys, times, transform, mut extrusion): Self::SystemData,
    ) {
        for (e, key, time, transform) in (&*entities, &keys, &times, &transform).join() {
            make_collides(key, time, transform, &store, &storage)
                .map(|collision| extrusion.entry(e).map(|entry| entry.or_insert(collision)));
        }
    }
}

fn make_collides<K>(
    key: &PlayAnimationKey<K>,
    time: &AnimationTime,
    transform: &Transform,
    store: &AnimationStore<K, AnimationParam>,
    storage: &AssetStorage<SpriteAnimation<AnimationParam>>,
) -> Option<ExtrudeCollision>
where
    K: AnimationKey,
{
    // これまで経過したアニメーションフレーム分の処理を行う
    // 直前のアニメーションフレームは前のフレームで処理してるので省く
    let mut global_matrixs = BTreeMap::new();
    let mut identity = Transform::default();
    identity.set_scale(*transform.scale());
    let root_scale_matrix = identity.matrix();
    let cubes = AnimationNodes::new(key.key()?, time.current_time(), &store, &storage)?
        .map(|(id, part_info, key_frame, _)| {
            let part_id = part_info.part_id();
            let parent_id = part_info.parent_id();

            // 親の位置からグローバル座標を算出．親がいなければルートが親
            let parent_matrix = parent_id
                .map(|parent_id| global_matrixs[&(id, parent_id)])
                .unwrap_or(root_scale_matrix);

            // グローバル座標計算
            let global_matrix = parent_matrix * key_frame.transform().matrix();

            // 後ろのパーツの計算のために BTreeMap にセット
            global_matrixs.insert((id, part_id), global_matrix);

            (part_info, key_frame, global_matrix)
        })
        .filter_map(|(part_info, key_frame, matrix)| {
            if key_frame.visible() {
                match (part_info.bounds(), key_frame.user()) {
                    (
                        Some(_),
                        Some(AnimationParam {
                            user_param:
                                Some(UserParamater {
                                    collision_type: Some(CollisionType::Extrusion),
                                }),
                            ..
                        }),
                    ) => Some(matrix),
                    _ => None,
                }
            } else {
                None
            }
        })
        .map(|matrix| {
            let collision: &[[f32; 4]; 4] = matrix.as_ref();
            let width = collision[0][0] / 2.;
            let height = collision[1][1] / 2.;
            let transform =
                Isometry2::new(Vector2::new(collision[3][0], collision[3][1]), math::zero());
            (
                transform,
                ShapeHandle::new(Cuboid::new(Vector2::new(width.abs(), height.abs()))),
            )
        })
        .collect::<Vec<_>>();

    if cubes.len() == 0 {
        ExtrudeCollision::none().into()
    } else {
        ExtrudeCollision::new(Compound::new(cubes)).into()
    }
}
