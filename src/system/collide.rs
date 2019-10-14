use crate::paramater::AnimationParam;
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
use ncollide2d::{
    query,
    shape::{Compound, Cuboid, ShapeHandle},
};
use std::{collections::BTreeMap, marker::PhantomData};

pub struct CollideSystem<K> {
    _key: PhantomData<K>,
}

struct CollisionData {
    collisions: Compound<f32>,
    transform: Isometry2<f32>,
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
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, store, storage, keys, times, mut transforms): Self::SystemData) {
        let mut object_diffs = BTreeMap::new();

        for (e1, key1, time1, transform1) in (&*entities, &keys, &times, &transforms).join() {
            for (e2, key2, time2, transform2) in (&*entities, &keys, &times, &transforms).join() {
                if e1 == e2 {
                    continue;
                }
                match (
                    make_collides(key1, time1, transform1, &store, &storage),
                    make_collides(key2, time2, transform2, &store, &storage),
                ) {
                    (Some(cuboids1), Some(cuboids2)) => {
                        contact(cuboids1, cuboids2).map(|diff| {
                                    object_diffs.insert(e1, diff);
                                });
                            }
                    _ => {}
                }
            }
        }

        for (e, diff) in object_diffs {
            if let Some(transform) = transforms.get_mut(e) {
                transform.append_translation_xyz(diff.0, diff.1, 0.0);
            }
        }
    }
}

fn make_collides<K>(
    key: &PlayAnimationKey<K>,
    time: &AnimationTime,
    transform: &Transform,
    store: &AnimationStore<K, AnimationParam>,
    storage: &AssetStorage<SpriteAnimation<AnimationParam>>,
) -> Option<CollisionData>
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
                part_info.bounds().map(|bounds| (bounds, key_frame, matrix))
            } else {
                None
            }
        })
        .map(|(_, _, matrix)| {
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
    CollisionData {
        collisions: Compound::new(cubes),
        transform: transform_to_isometry(transform),
    }
        .into()
}

fn contact(c1: CollisionData, c2: CollisionData) -> Option<(f32, f32)> {
    let contact = query::contact(
        &c1.transform,
        &c1.collisions,
        &c2.transform,
        &c2.collisions,
        2.0,
    );
    match contact {
        Some(contact) => {
            let query::Contact { normal, depth, .. } = contact;
            let diff_x = normal.into_inner()[0] * -depth / 2.;
            let diff_y = normal.into_inner()[1] * -depth / 2.;
            Some((diff_x, diff_y))
        }
        None => None,
    }
}

fn transform_to_isometry(transform: &Transform) -> Isometry2<f32> {
    let matrix = transform.matrix();
    let collision: &[[f32; 4]; 4] = matrix.as_ref();
    Isometry2::new(Vector2::new(collision[3][0], collision[3][1]), math::zero())
}
