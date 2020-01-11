use crate::{
    components::Collisions,
    paramater::AnimationParam,
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
    resource::{data::AnimationData, AnimationStore},
    traits::{AnimationKey, FileId},
};
use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
enum RegisterError {
    NotFoundPack,
    NotFoundAnimation,
    EntryError(WrongGeneration),
}

pub struct RegisterColliderSystem<ID, P, A, C, T> {
    _file_id: PhantomData<ID>,
    _pack_id: PhantomData<P>,
    _animation_id: PhantomData<A>,
    _paramater: PhantomData<C>,
    _collision: PhantomData<T>,
}

impl<ID, P, A, C, T> RegisterColliderSystem<ID, P, A, C, T> {
    pub fn new() -> Self {
        RegisterColliderSystem {
            _file_id: PhantomData,
            _pack_id: PhantomData,
            _animation_id: PhantomData,
            _paramater: PhantomData,
            _collision: PhantomData,
        }
    }
}

impl<'s, ID, P, A, C, T> System<'s> for RegisterColliderSystem<ID, P, A, C, T>
where
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    T: 'static + Send + Sync + ParamaterFromData<AnimationParam>,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, AnimationStore<ID, AnimationParam>>,
        Read<'s, AssetStorage<AnimationData<AnimationParam>>>,
        ReadStorage<'s, PlayAnimationKey<ID, P, A>>,
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

fn register_collision<C, T, ID, P, A>(
    e: Entity,
    key: &PlayAnimationKey<ID, P, A>,
    time: &AnimationTime,
    root_transform: &Transform,
    collisions: &mut WriteStorage<Collisions<C, T>>,
    animation_store: &Read<AnimationStore<ID, AnimationParam>>,
    sprite_animation_storage: &Read<AssetStorage<AnimationData<AnimationParam>>>,
) -> Result<(), RegisterError>
where
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    T: 'static + Send + Sync + ParamaterFromData<AnimationParam>,
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
{
    let registered_collision = collisions
        .entry(e)
        .map_err(|err| RegisterError::EntryError(err))?
        .or_insert(Collisions::new());
    let (id, &pack_id, &animation_id) = match (key.file_id(), key.pack_name(), key.animation_name())
    {
        (id, Some(pack), Some(anim)) => (id, pack, anim),
        _ => {
            return Ok(());
        }
    };
    let pack_name = pack_id.to_string();
    let animation_name = animation_id.to_string();
    let pack = animation_store
        .get_animation_handle(id)
        .and_then(|handle| sprite_animation_storage.get(handle))
        .and_then(|data| data.pack(&pack_name))
        .ok_or(RegisterError::NotFoundPack)?;

    let animation = pack
        .animation(&animation_name)
        .ok_or(RegisterError::NotFoundAnimation)?;
    let frame = animation.sec_to_frame(time.current_time());
    let mut global_transforms = BTreeMap::new();
    pack.parts().enumerate().for_each(|(part_id, part)| {
        let parent_id = part.parent_id();
        let hash_key = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            pack_id.hash(&mut hasher);
            animation_id.hash(&mut hasher);
            part_id.hash(&mut hasher);
            hasher.finish()
        };

        // 親の位置を取得してからパーツのローカル座標を反映
        // 後で判定登録のときに座標計算するので基本は 0 座標
        let mut part_transform = parent_id
            .and_then(|parent_id| {
                let hash_key = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    pack_id.hash(&mut hasher);
                    animation_id.hash(&mut hasher);
                    parent_id.hash(&mut hasher);
                    hasher.finish()
                };
                global_transforms.get(&hash_key)
            })
            .unwrap_or(root_transform)
            .clone();
        part_transform.concat(&animation.local_transform(part_id, frame));

        let user = animation.user(part_id, frame);
        if let Some(AnimationParam {
            collision_type: Some(_),
            ..
        }) = user
        {
            let c = C::make_collision(&part_transform);
            // キーフレームから collisions に登録するデータを生成するトレイト
            let data = T::make_collision_data(user.unwrap());

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
            match registered_collision.remove_collision((pack_id, part_id)) {
                Some((id, handle)) => {
                    log::error!("removed {:?}, {:?}", id, handle);
                }
                None => {}
            }
        }

        // 今のIDで位置を登録しておき，次の子パーツの座標計算に利用する
        global_transforms.insert(hash_key, part_transform);
    });
    Ok(())
}
