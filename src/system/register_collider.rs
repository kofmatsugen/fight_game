use crate::{
    components::Collisions,
    paramater::{AnimationParam, KeyParamater},
    traits::{CollisionData, CollisionFromData, ParamaterFromData},
};
use amethyst::{
    assets::AssetStorage,
    core::{
        components::Transform,
        ecs::{
            error::WrongGeneration, Entities, Entity, Join, Read, ReadStorage, System, WriteStorage,
        },
    },
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    iter::AnimationNodes,
    resource::AnimationStore,
    traits::AnimationKey,
    types::{KeyFrame, Node},
    SpriteAnimation,
};
use std::{collections::BTreeMap, marker::PhantomData};

pub struct RegisterColliderSystem<K, C, T> {
    _key: PhantomData<K>,
    _paramater: PhantomData<C>,
    _collision: PhantomData<T>,
}

impl<K, C, T> RegisterColliderSystem<K, C, T> {
    pub fn new() -> Self {
        RegisterColliderSystem {
            _key: PhantomData,
            _paramater: PhantomData,
            _collision: PhantomData,
        }
    }
}

impl<'s, K, C, T> System<'s> for RegisterColliderSystem<K, C, T>
where
    K: AnimationKey,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    T: 'static + Send + Sync + ParamaterFromData<KeyFrame<AnimationParam>>,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, AnimationStore<K, AnimationParam>>,
        Read<'s, AssetStorage<SpriteAnimation<AnimationParam>>>,
        ReadStorage<'s, PlayAnimationKey<K>>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Collisions<C, T>>,
    );

    fn run(
        &mut self,
        (entities, store, storage, keys, times, transforms, mut collisions): Self::SystemData,
    ) {
        for (e, key, time, transform) in (&*entities, &keys, &times, &transforms).join() {
            match register_collision(e, key, time, transform, &mut collisions, &store, &storage) {
                Ok(()) => {}
                Err(err) => log::error!("{:?}", err),
            }
        }
    }
}

fn register_collision<C, T, K>(
    e: Entity,
    key: &PlayAnimationKey<K>,
    time: &AnimationTime,
    root_transform: &Transform,
    collisions: &mut WriteStorage<Collisions<C, T>>,
    store: &Read<AnimationStore<K, AnimationParam>>,
    storage: &Read<AssetStorage<SpriteAnimation<AnimationParam>>>,
) -> Result<(), WrongGeneration>
where
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    T: 'static + Send + Sync + ParamaterFromData<KeyFrame<AnimationParam>>,
    K: AnimationKey,
{
    let registered_collision = collisions.entry(e)?.or_insert(Collisions::new());
    let mut global_transforms = BTreeMap::new();
    if let Some(anim_key) = key.key() {
        for Node {
            pack_id,
            part_info,
            key_frame,
            ..
        } in AnimationNodes::new(anim_key, time.current_time(), &store, &storage).unwrap()
        {
            let part_id = part_info.part_id();
            let parent_id = part_info.parent_id();

            // 親の位置を取得してからパーツのローカル座標を反映
            // 後で判定登録のときに座標計算するので基本は 0 座標
            let mut part_transform = parent_id
                .and_then(|parent_id| global_transforms.get(&(pack_id, parent_id)))
                .unwrap_or(root_transform)
                .clone();
            part_transform.concat(key_frame.transform());

            if let Some(AnimationParam {
                key_param:
                    Some(KeyParamater {
                        collision_type: Some(_),
                    }),
                ..
            }) = key_frame.user()
            {
                let c = C::make_collision(&part_transform);
                // キーフレームから collisions に登録するデータを生成するトレイト
                let data = T::make_collision_data(key_frame);

                log::info!(
                    "register {:?}: {:?}, {:?}",
                    (pack_id, part_id),
                    c,
                    part_transform.translation()
                );
                registered_collision.update_collision(
                    (pack_id, part_id),
                    data,
                    c,
                    part_transform.clone(),
                );
            } else {
                // 判定データがないので削除
                log::info!("erase {:?}", (pack_id, part_id),);
                registered_collision.remove_collision((pack_id, part_id));
            }

            // 今のIDで位置を登録しておき，次の子パーツの座標計算に利用する
            global_transforms.insert((pack_id, part_id), part_transform);
        }
    }
    Ok(())
}
