use crate::{
    components::Collisions,
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
    traits::animation_file::AnimationFile,
};
use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
enum RegisterError<T>
where
    T: AnimationFile,
{
    NotFoundPack(T::PackKey),
    NotFoundAnimation(T::AnimationKey),
    EntryError(WrongGeneration),
}

pub struct RegisterColliderSystem<T, C, P> {
    _animation_file: PhantomData<T>,
    _paramater: PhantomData<C>,
    _collision: PhantomData<P>,
}

impl<T, C, P> RegisterColliderSystem<T, C, P> {
    pub fn new() -> Self {
        RegisterColliderSystem {
            _animation_file: PhantomData,
            _paramater: PhantomData,
            _collision: PhantomData,
        }
    }
}

impl<'s, T, C, P> System<'s> for RegisterColliderSystem<T, C, P>
where
    T: AnimationFile + std::fmt::Debug,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, AnimationStore<T>>,
        Read<'s, AssetStorage<AnimationData<T>>>,
        ReadStorage<'s, PlayAnimationKey<T>>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Collisions<C, P>>,
    );

    fn run(
        &mut self,
        (entities, store, storage, keys, times, transforms, mut collisions): Self::SystemData,
    ) {
        for (e, key, time, transform) in (&*entities, &keys, &times, &transforms).join() {
            match register_collision::<T, _, _>(
                e,
                key,
                time,
                transform,
                &mut collisions,
                &store,
                &storage,
            ) {
                Ok(()) => {}
                Err(err) => log::error!("{:?}", err),
            }
        }
    }
}

fn register_collision<T, C, P>(
    e: Entity,
    key: &PlayAnimationKey<T>,
    time: &AnimationTime,
    root_transform: &Transform,
    collisions: &mut WriteStorage<Collisions<C, P>>,
    animation_store: &Read<AnimationStore<T>>,
    sprite_animation_storage: &Read<AssetStorage<AnimationData<T>>>,
) -> Result<(), RegisterError<T>>
where
    T: AnimationFile + std::fmt::Debug,
    C: 'static + Send + Sync + CollisionData + CollisionFromData<Transform> + std::fmt::Debug,
    P: 'static + Send + Sync + ParamaterFromData<T::UserData>,
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
    let pack = animation_store
        .get_animation_handle(id)
        .and_then(|handle| sprite_animation_storage.get(handle))
        .and_then(|data| data.pack(&pack_id))
        .ok_or(RegisterError::NotFoundPack(pack_id))?;

    let animation = pack
        .animation(&animation_id)
        .ok_or(RegisterError::NotFoundAnimation(animation_id))?;
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
        let c = C::make_collision(&part_transform);
        // キーフレームから collisions に登録するデータを生成するトレイト
        let data = P::make_collision_data(user);
        if let Some(data) = data {
            log::trace!(
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
                    log::trace!("remove collision {:?}, {:?}", id, handle);
                }
                None => {}
            }
        }

        // 今のIDで位置を登録しておき，次の子パーツの座標計算に利用する
        global_transforms.insert(hash_key, part_transform);
    });
    Ok(())
}
